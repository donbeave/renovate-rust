//! NuGet `.csproj` / `.props` / `.targets` dependency extractor.
//!
//! Parses MSBuild XML project files using the SAX-style `quick-xml` reader and
//! returns `<PackageReference>` and related elements with their version strings.
//!
//! Renovate reference:
//! - `lib/modules/manager/nuget/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/nuget/util.ts`    — `findVersion`
//!
//! ## Supported elements
//!
//! | Element | Version attribute |
//! |---|---|
//! | `<PackageReference Include/Update="…" Version="…">` | `Version`, `VersionOverride` |
//! | `<PackageVersion Include="…" Version="…">` | `Version` |
//! | `<DotNetCliToolReference Include="…" Version="…">` | `Version` |
//! | `<GlobalPackageReference Include="…" Version="…">` | `Version` |
//!
//! ## Version forms
//!
//! | Form | Treatment |
//! |---|---|
//! | `1.2.3` | Actionable — exact version |
//! | `[1.2.3]` | Actionable — exact NuGet range |
//! | `[1.2.3,]` or `[1.2.3,)` | Actionable — minimum-only range |
//! | `$(Variable)` | Skipped — `PropertyRef` |
//! | `[,1.2.3)` or range with upper bound | Skipped — `VersionRange` |

use std::collections::HashMap;
use std::io::BufReader;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use serde_json::Value;
use thiserror::Error;

/// Which MSBuild element the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NuGetDepType {
    /// `<PackageReference>`
    Package,
    /// `<PackageVersion>` (Directory.Packages.props)
    PackageVersion,
    /// `<DotNetCliToolReference>`
    CliTool,
    /// `<GlobalPackageReference>`
    Global,
    /// MSBuild SDK reference (`<Project Sdk="…/version">`, `<Sdk Name="…" Version="…">`, `<Import Sdk="…" Version="…">`)
    MsbuildSdk,
    /// .NET SDK version from `global.json`.
    DotnetSdk,
    /// Local .NET tool manifest entry from `.config/dotnet-tools.json`.
    DotnetTool,
    /// `#:package` directive in a single C# file.
    SingleFilePackage,
    /// `<ContainerBaseImage>` property — Docker image for .NET container publishing.
    ContainerImage,
}

impl NuGetDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            NuGetDepType::Package => "PackageReference",
            NuGetDepType::PackageVersion => "PackageVersion",
            NuGetDepType::CliTool => "DotNetCliToolReference",
            NuGetDepType::Global => "GlobalPackageReference",
            NuGetDepType::MsbuildSdk => "msbuild-sdk",
            NuGetDepType::DotnetSdk => "dotnet-sdk",
            NuGetDepType::DotnetTool => "nuget",
            NuGetDepType::SingleFilePackage => "nuget",
            NuGetDepType::ContainerImage => "docker",
        }
    }
}

/// Why a NuGet package reference is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NuGetSkipReason {
    /// Version is an MSBuild property reference `$(…)`.
    PropertyRef,
    /// Version is a NuGet version range with an upper bound or exclusive lower
    /// bound that we cannot safely update.
    VersionRange,
    /// No version was specified.
    NoVersion,
}

/// A single extracted NuGet dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NuGetExtractedDep {
    /// Package ID (case-preserved; NuGet is case-insensitive).
    pub package_id: String,
    /// Version string, normalized from range notation where possible.
    pub current_value: String,
    /// Digest for digest-pinned container images.
    pub current_digest: Option<String>,
    /// Which element type this came from.
    pub dep_type: NuGetDepType,
    pub registry_urls: Vec<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<NuGetSkipReason>,
}

/// Extracted content from an MSBuild `global.json` manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NuGetGlobalJsonExtract {
    pub deps: Vec<NuGetExtractedDep>,
    pub dotnet_sdk_constraint: Option<String>,
}

/// Extracted content from an MSBuild project file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NuGetProjectExtract {
    pub deps: Vec<NuGetExtractedDep>,
    pub package_file_version: Option<String>,
    pub lock_files: Vec<String>,
}

/// Errors from parsing a NuGet project file.
#[derive(Debug, Error)]
pub enum NuGetExtractError {
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a NuGet project file and extract all package references.
pub fn extract(content: &str) -> Result<Vec<NuGetExtractedDep>, NuGetExtractError> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut deps = Vec::new();
    let mut buf = Vec::new();

    // Pending dep built from a `<PackageReference>` Start event.
    let mut current: Option<PendingDep> = None;
    // True while inside a <Version> or <VersionOverride> child element.
    let mut version_child_tag = Option::<String>::None;
    // True while inside a <ContainerBaseImage> child element.
    let mut in_container_image = false;

    loop {
        match reader.read_event_into(&mut buf)? {
            // Self-closing element: emit immediately.
            Event::Empty(ref e) => {
                let elem = elem_name(e);
                // MSBuild SDK: `<Sdk Name="..." Version="...">` or `<Import Sdk="..." Version="...">`
                if elem == "Sdk" || elem == "Import" {
                    if let Some(pending) = attrs_to_sdk_pending(e) {
                        deps.push(build_dep(pending));
                    }
                } else if let Some(dep_type) = dep_type_for(&elem)
                    && let Some(pending) = attrs_to_pending(e, dep_type)
                    && !pending.package_id.is_empty()
                {
                    deps.push(build_dep(pending));
                }
            }

            // Opening element: may have child elements.
            Event::Start(ref e) => {
                let elem = elem_name(e);
                // MSBuild SDK: `<Project Sdk="Name/Version">` on the root element
                if elem == "Project"
                    && let Some(pending) = attrs_to_project_sdk(e)
                {
                    deps.push(build_dep(pending));
                }
                if elem == "ContainerBaseImage" {
                    in_container_image = true;
                } else if let Some(dep_type) = dep_type_for(&elem) {
                    current = attrs_to_pending(e, dep_type);
                } else if current.is_some() && (elem == "Version" || elem == "VersionOverride") {
                    version_child_tag = Some(elem);
                }
            }

            Event::End(e) => {
                let elem = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if elem == "ContainerBaseImage" {
                    in_container_image = false;
                } else if version_child_tag.as_deref() == Some(elem.as_str()) {
                    version_child_tag = None;
                } else if dep_type_for(&elem).is_some()
                    && let Some(pending) = current.take()
                    && !pending.package_id.is_empty()
                {
                    deps.push(build_dep(pending));
                }
            }

            Event::Text(e) => {
                let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                if in_container_image && !text.is_empty() {
                    let (image, tag, digest) = parse_container_image(&text);
                    if !image.is_empty() && !tag.is_empty() {
                        deps.push(NuGetExtractedDep {
                            package_id: image,
                            current_value: tag,
                            current_digest: digest,
                            dep_type: NuGetDepType::ContainerImage,
                            registry_urls: Vec::new(),
                            skip_reason: None,
                        });
                    }
                } else if let Some(ref tag) = version_child_tag
                    && !text.is_empty()
                    && let Some(ref mut pending) = current
                    && (tag == "VersionOverride" || pending.version.is_empty())
                {
                    pending.version = text;
                }
            }

            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(deps)
}

/// Parse a NuGet project file and include package-file level metadata.
pub fn extract_project_file(
    content: &str,
    package_file: &str,
    lock_file_exists: bool,
) -> Result<Option<NuGetProjectExtract>, NuGetExtractError> {
    extract_project_file_with_config(content, package_file, lock_file_exists, &[])
}

pub fn extract_project_file_with_config(
    content: &str,
    package_file: &str,
    lock_file_exists: bool,
    files: &[(&str, Option<&str>)],
) -> Result<Option<NuGetProjectExtract>, NuGetExtractError> {
    let mut deps = extract(content)?;
    if deps.is_empty() {
        return Ok(None);
    }

    apply_config_registry_urls(&mut deps, package_file, files);

    let package_file_version = extract_package_file_version(content)?;
    let lock_files = if lock_file_exists {
        vec![sibling_file_name(package_file, "packages.lock.json")]
    } else {
        Vec::new()
    };

    Ok(Some(NuGetProjectExtract {
        deps,
        package_file_version,
        lock_files,
    }))
}

/// Parse a `.config/dotnet-tools.json` local tool manifest.
///
/// Renovate treats unsupported manifest versions, missing `tools`, and invalid
/// JSON as null extraction results; this tolerant API mirrors that behavior by
/// returning an empty dependency list for those cases.
pub fn extract_dotnet_tools(content: &str) -> Vec<NuGetExtractedDep> {
    extract_dotnet_tools_with_config(content, "", &[])
}

pub fn extract_dotnet_tools_with_config(
    content: &str,
    package_file: &str,
    files: &[(&str, Option<&str>)],
) -> Vec<NuGetExtractedDep> {
    let Ok(manifest) = serde_json::from_str::<Value>(content) else {
        return Vec::new();
    };

    if manifest.get("version").and_then(Value::as_u64) != Some(1) {
        return Vec::new();
    }

    let Some(tools) = manifest.get("tools").and_then(Value::as_object) else {
        return Vec::new();
    };

    let mut deps: Vec<_> = tools
        .iter()
        .filter_map(|(name, tool)| {
            let version = tool.get("version").and_then(Value::as_str)?;
            if version.is_empty() {
                return None;
            }
            Some(NuGetExtractedDep {
                package_id: name.clone(),
                current_value: version.to_owned(),
                current_digest: None,
                dep_type: NuGetDepType::DotnetTool,
                registry_urls: Vec::new(),
                skip_reason: None,
            })
        })
        .collect();
    apply_config_registry_urls(&mut deps, package_file, files);
    deps
}

/// Parse NuGet directives from a .NET 10 single C# file.
///
/// Supports Renovate's current directive forms:
/// `#:sdk Name@Version` and `#:package Name@Version`.
pub fn extract_single_csharp_file(content: &str) -> Vec<NuGetExtractedDep> {
    extract_single_csharp_file_with_config(content, "", &[])
}

pub fn extract_single_csharp_file_with_config(
    content: &str,
    package_file: &str,
    files: &[(&str, Option<&str>)],
) -> Vec<NuGetExtractedDep> {
    let mut deps: Vec<_> = content
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let (dep_type, rest) = if let Some(rest) = line.strip_prefix("#:sdk ") {
                (NuGetDepType::MsbuildSdk, rest)
            } else if let Some(rest) = line.strip_prefix("#:package ") {
                (NuGetDepType::SingleFilePackage, rest)
            } else {
                return None;
            };

            let (name, version) = rest.split_once('@')?;
            if name.is_empty()
                || version.is_empty()
                || !version.starts_with(|ch: char| ch.is_ascii_digit())
                || !name
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '-'))
            {
                return None;
            }

            let current_value = version.split_whitespace().next().unwrap_or_default();
            if current_value.is_empty() {
                return None;
            }

            Some(NuGetExtractedDep {
                package_id: name.to_owned(),
                current_value: current_value.to_owned(),
                current_digest: None,
                dep_type,
                registry_urls: Vec::new(),
                skip_reason: None,
            })
        })
        .collect();
    apply_config_registry_urls(&mut deps, package_file, files);
    deps
}

/// Parse a NuGet-relevant `global.json` manifest.
///
/// Invalid JSON and manifests with neither `sdk.version` nor `msbuild-sdks`
/// return `None`, matching Renovate's null extraction result.
pub fn extract_global_json(content: &str) -> Option<NuGetGlobalJsonExtract> {
    let manifest = serde_json::from_str::<Value>(content).ok()?;
    let mut deps = Vec::new();
    let mut dotnet_sdk_constraint = None;

    if let Some(version) = manifest
        .get("sdk")
        .and_then(|sdk| sdk.get("version"))
        .and_then(Value::as_str)
    {
        dotnet_sdk_constraint = Some(version.to_owned());
        deps.push(NuGetExtractedDep {
            package_id: "dotnet-sdk".to_owned(),
            current_value: version.to_owned(),
            current_digest: None,
            dep_type: NuGetDepType::DotnetSdk,
            registry_urls: Vec::new(),
            skip_reason: None,
        });
    }

    if let Some(msbuild_sdks) = manifest.get("msbuild-sdks").and_then(Value::as_object) {
        for (name, version) in msbuild_sdks {
            if let Some(version) = version.as_str() {
                deps.push(NuGetExtractedDep {
                    package_id: name.clone(),
                    current_value: version.to_owned(),
                    current_digest: None,
                    dep_type: NuGetDepType::MsbuildSdk,
                    registry_urls: Vec::new(),
                    skip_reason: None,
                });
            }
        }
    }

    if deps.is_empty() {
        None
    } else {
        Some(NuGetGlobalJsonExtract {
            deps,
            dotnet_sdk_constraint,
        })
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn sibling_file_name(package_file: &str, sibling: &str) -> String {
    if let Some((dir, _)) = package_file.rsplit_once('/') {
        format!("{dir}/{sibling}")
    } else {
        sibling.to_owned()
    }
}

fn nuget_config_registry_urls(package_file: &str, files: &[(&str, Option<&str>)]) -> Vec<String> {
    let file_contents: HashMap<&str, &str> = files
        .iter()
        .filter_map(|(path, content)| content.map(|content| (*path, content)))
        .collect();

    for dir in package_file_dirs(package_file) {
        for config_name in ["nuget.config", "NuGet.config", "NuGet.Config"] {
            let path = join_path(&dir, config_name);
            if let Some(content) = file_contents.get(path.as_str()) {
                return parse_nuget_config_registry_urls(content);
            }
        }
    }

    Vec::new()
}

fn apply_config_registry_urls(
    deps: &mut [NuGetExtractedDep],
    package_file: &str,
    files: &[(&str, Option<&str>)],
) {
    let registry_urls = nuget_config_registry_urls(package_file, files);
    if registry_urls.is_empty() {
        return;
    }

    for dep in deps {
        if matches!(
            dep.dep_type,
            NuGetDepType::Package
                | NuGetDepType::PackageVersion
                | NuGetDepType::CliTool
                | NuGetDepType::Global
                | NuGetDepType::DotnetTool
                | NuGetDepType::SingleFilePackage
                | NuGetDepType::MsbuildSdk
        ) {
            dep.registry_urls.clone_from(&registry_urls);
        }
    }
}

fn package_file_dirs(package_file: &str) -> Vec<String> {
    let Some((mut dir, _)) = package_file.rsplit_once('/') else {
        return vec![String::new()];
    };
    let mut dirs = Vec::new();
    loop {
        dirs.push(dir.to_owned());
        let Some((parent, _)) = dir.rsplit_once('/') else {
            dirs.push(String::new());
            break;
        };
        dir = parent;
    }
    dirs
}

fn join_path(dir: &str, file: &str) -> String {
    if dir.is_empty() {
        file.to_owned()
    } else {
        format!("{dir}/{file}")
    }
}

fn parse_nuget_config_registry_urls(content: &str) -> Vec<String> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);
    let mut urls = Vec::new();
    let mut in_package_sources = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if elem_name(&e) == "packageSources" => {
                in_package_sources = true;
            }
            Ok(Event::End(e)) if String::from_utf8_lossy(e.name().as_ref()) == "packageSources" => {
                in_package_sources = false;
            }
            Ok(Event::Empty(e) | Event::Start(e))
                if in_package_sources && elem_name(&e) == "add" =>
            {
                if let Some(url) = attr_value(&e, "value")
                    && (url.starts_with("https://") || url.starts_with("http://"))
                {
                    urls.push(url);
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    urls
}

fn attr_value(e: &BytesStart<'_>, name: &str) -> Option<String> {
    for attr in e.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref());
        if key.eq_ignore_ascii_case(name) {
            return Some(attr.unescape_value().unwrap_or_default().into_owned());
        }
    }
    None
}

fn extract_package_file_version(content: &str) -> Result<Option<String>, NuGetExtractError> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<String> = Vec::new();
    let mut capture_version = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => {
                let elem = elem_name(e);
                stack.push(elem.clone());
                if stack.len() == 3
                    && stack[0] == "Project"
                    && stack[1] == "PropertyGroup"
                    && (elem == "Version" || elem == "VersionPrefix")
                {
                    capture_version = true;
                }
            }
            Event::End(e) => {
                let elem = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if capture_version && (elem == "Version" || elem == "VersionPrefix") {
                    capture_version = false;
                }
                stack.pop();
            }
            Event::Text(e) if capture_version => {
                let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                if !text.is_empty() {
                    return Ok(Some(text));
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(None)
}

struct PendingDep {
    package_id: String,
    version: String,
    dep_type: NuGetDepType,
}

fn elem_name(e: &BytesStart<'_>) -> String {
    String::from_utf8_lossy(e.name().as_ref()).into_owned()
}

/// Parse MSBuild SDK from `<Project Sdk="Name/Version">`.
fn attrs_to_project_sdk(e: &BytesStart<'_>) -> Option<PendingDep> {
    for attr in e.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
        if key.to_lowercase() != "sdk" {
            continue;
        }
        let val = attr.unescape_value().unwrap_or_default().into_owned();
        // Format: "Name/Version" — version after the last slash.
        let slash = val.rfind('/')?;
        let name = val[..slash].to_owned();
        let version = val[slash + 1..].to_owned();
        if name.is_empty() || version.is_empty() {
            return None;
        }
        return Some(PendingDep {
            package_id: name,
            version,
            dep_type: NuGetDepType::MsbuildSdk,
        });
    }
    None
}

/// Parse MSBuild SDK from `<Sdk Name="..." Version="...">` or `<Import Sdk="..." Version="...">`.
fn attrs_to_sdk_pending(e: &BytesStart<'_>) -> Option<PendingDep> {
    let mut name = String::new();
    let mut version = String::new();

    for attr in e.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
        let val = attr.unescape_value().unwrap_or_default().into_owned();
        match key.to_lowercase().as_str() {
            "name" | "sdk" if name.is_empty() => {
                name = val;
            }
            "version" => version = val,
            _ => {}
        }
    }

    if name.is_empty() || version.is_empty() {
        return None;
    }

    Some(PendingDep {
        package_id: name,
        version,
        dep_type: NuGetDepType::MsbuildSdk,
    })
}

fn dep_type_for(elem: &str) -> Option<NuGetDepType> {
    match elem {
        "PackageReference" => Some(NuGetDepType::Package),
        "PackageVersion" => Some(NuGetDepType::PackageVersion),
        "DotNetCliToolReference" => Some(NuGetDepType::CliTool),
        "GlobalPackageReference" => Some(NuGetDepType::Global),
        _ => None,
    }
}

fn attrs_to_pending(e: &BytesStart<'_>, dep_type: NuGetDepType) -> Option<PendingDep> {
    let mut package_id = String::new();
    let mut version = String::new();
    let mut version_override = String::new();

    for attr in e.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
        let val = attr.unescape_value().unwrap_or_default().into_owned();
        match key.to_lowercase().as_str() {
            "include" | "update" => package_id = val,
            "version" => version = val,
            "versionoverride" => version_override = val,
            _ => {}
        }
    }

    if package_id.is_empty() {
        return None;
    }

    let resolved_version = if !version_override.is_empty() {
        version_override
    } else {
        version
    };

    Some(PendingDep {
        package_id,
        version: resolved_version,
        dep_type,
    })
}

fn build_dep(dep: PendingDep) -> NuGetExtractedDep {
    if dep.version.is_empty() {
        return NuGetExtractedDep {
            package_id: dep.package_id,
            current_value: String::new(),
            current_digest: None,
            dep_type: dep.dep_type,
            registry_urls: Vec::new(),
            skip_reason: Some(NuGetSkipReason::NoVersion),
        };
    }

    if dep.version.contains("$(") {
        return NuGetExtractedDep {
            package_id: dep.package_id,
            current_value: dep.version,
            current_digest: None,
            dep_type: dep.dep_type,
            registry_urls: Vec::new(),
            skip_reason: Some(NuGetSkipReason::PropertyRef),
        };
    }

    let (current_value, skip_reason) = normalize_version(&dep.version);
    NuGetExtractedDep {
        package_id: dep.package_id,
        current_value,
        current_digest: None,
        dep_type: dep.dep_type,
        registry_urls: Vec::new(),
        skip_reason,
    }
}

/// Parse a ContainerBaseImage value like `image:tag@digest` or `image:tag`.
/// Returns `(image_name, tag, digest)`.
fn parse_container_image(s: &str) -> (String, String, Option<String>) {
    let (image_with_tag, digest) = if let Some(at_pos) = s.find('@') {
        (&s[..at_pos], Some(s[at_pos + 1..].to_owned()))
    } else {
        (s, None)
    };
    if let Some(colon_pos) = image_with_tag.rfind(':') {
        let tag = &image_with_tag[colon_pos + 1..];
        if !tag.contains('/') {
            let image = &image_with_tag[..colon_pos];
            return (image.to_owned(), tag.to_owned(), digest);
        }
    }
    (image_with_tag.to_owned(), String::new(), digest)
}

/// Normalize a NuGet version string.
fn normalize_version(v: &str) -> (String, Option<NuGetSkipReason>) {
    let trimmed = v.trim();

    if !trimmed.starts_with('[') && !trimmed.starts_with('(') {
        return (trimmed.to_owned(), None);
    }

    let inner = trimmed
        .trim_start_matches(['[', '('])
        .trim_end_matches([']', ')'])
        .trim();

    if inner.contains(',') {
        let mut parts = inner.splitn(2, ',');
        let lower = parts.next().unwrap_or("").trim();
        let upper = parts.next().unwrap_or("").trim();

        // Only inclusive-minimum ranges (`[lower,]` / `[lower,)`) are updatable.
        // Exclusive-minimum (`(lower,)`) and any upper-bound ranges are skipped.
        if upper.is_empty() && !lower.is_empty() && trimmed.starts_with('[') {
            return (lower.to_owned(), None);
        }
        return (trimmed.to_owned(), Some(NuGetSkipReason::VersionRange));
    }

    if !inner.is_empty() {
        return (inner.to_owned(), None);
    }

    (trimmed.to_owned(), Some(NuGetSkipReason::VersionRange))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<NuGetExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn simple_package_reference() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
    <PackageReference Include="Serilog" Version="3.1.1" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        let json = deps
            .iter()
            .find(|d| d.package_id == "Newtonsoft.Json")
            .unwrap();
        assert_eq!(json.current_value, "13.0.3");
        assert!(json.skip_reason.is_none());
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn update_attribute_extracted() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Update="Autofac" Version="4.5.0" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "Autofac");
        assert_eq!(deps[0].current_value, "4.5.0");
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn version_override_attribute_wins() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Include="Newtonsoft.Json" Version="10.0.2" VersionOverride="13.0.3" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].current_value, "13.0.3");
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn version_child_element() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Include="Serilog">
      <VersionOverride>2.4.0</VersionOverride>
    </PackageReference>
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "2.4.0");
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn property_ref_skipped() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Include="Autofac" Version="$(AutofacVersion)" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(NuGetSkipReason::PropertyRef));
    }

    // Ported: "does not fail on package file without version" — nuget/extract.spec.ts line 79
    #[test]
    fn no_version_skipped() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Include="Serilog" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(NuGetSkipReason::NoVersion));
    }

    // Ported: "extracts package version dependency" — nuget/extract.spec.ts line 61
    #[test]
    fn package_version_dependency_extracted() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageVersion Include="Autofac" Version="4.5.0" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "Autofac");
        assert_eq!(deps[0].current_value, "4.5.0");
        assert_eq!(deps[0].dep_type, NuGetDepType::PackageVersion);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts package file version" — nuget/extract.spec.ts line 70
    #[test]
    fn package_file_version_and_lock_file_extracted() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk.Web">
  <PropertyGroup>
    <TargetFramework>netcoreapp1.1</TargetFramework>
    <Version>0.1.0</Version>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Microsoft.AspNetCore.Mvc.Core" Version="1.1.3" />
  </ItemGroup>
</Project>"#;
        let extracted = extract_project_file(content, "sample.csproj", true)
            .expect("project should parse")
            .expect("project should contain deps");
        assert_eq!(extracted.package_file_version, Some("0.1.0".to_owned()));
        assert_eq!(extracted.lock_files, vec!["packages.lock.json"]);
        assert_eq!(extracted.deps.len(), 1);
        assert_eq!(
            extracted.deps[0].package_id,
            "Microsoft.AspNetCore.Mvc.Core"
        );
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn exact_nuget_range_normalized() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Include="Range1" Version="[1.2.3]" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].current_value, "1.2.3");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn minimum_only_range_normalized() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Include="R2" Version="[1.2.3,]" />
    <PackageReference Include="R3" Version="[1.2.3,)" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        for dep in &deps {
            assert_eq!(dep.current_value, "1.2.3");
            assert!(dep.skip_reason.is_none());
        }
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn upper_bound_range_skipped() {
        let content = r#"<Project>
  <ItemGroup>
    <PackageReference Include="N1" Version="[,1.2.3)" />
    <PackageReference Include="N2" Version="(1.2.3,)" />
    <PackageReference Include="N3" Version="[1.2.3, 3.2.1]" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        for dep in &deps {
            assert_eq!(dep.skip_reason, Some(NuGetSkipReason::VersionRange));
        }
    }

    // Ported: "extracts all dependencies from global packages file" — nuget/extract.spec.ts line 226
    #[test]
    fn global_and_cli_tool_references() {
        let content = r#"<Project>
  <ItemGroup>
    <GlobalPackageReference Include="Roslynator.Analyzers" Version="2.0.0" />
    <DotNetCliToolReference Include="dotnet-ef" Version="7.0.0" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.dep_type == NuGetDepType::Global
                    && d.package_id == "Roslynator.Analyzers")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_type == NuGetDepType::CliTool && d.package_id == "dotnet-ef")
        );
    }

    // Ported: "does not fail on package file without version" — nuget/extract.spec.ts line 79
    #[test]
    fn empty_project_returns_empty() {
        let deps = extract_ok(r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup/></Project>"#);
        assert!(deps.is_empty());
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn normalize_plain_version() {
        assert_eq!(normalize_version("1.2.3"), ("1.2.3".to_owned(), None));
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn normalize_exact_range() {
        assert_eq!(normalize_version("[1.2.3]"), ("1.2.3".to_owned(), None));
    }

    // Ported: "extracts all dependencies" — nuget/extract.spec.ts line 86
    #[test]
    fn normalize_minimum_ranges() {
        assert_eq!(normalize_version("[1.2.3,]"), ("1.2.3".to_owned(), None));
        assert_eq!(normalize_version("[1.2.3,)"), ("1.2.3".to_owned(), None));
    }

    // Ported: "extracts dependency with lower-case Version attribute" — nuget/extract.spec.ts line 212
    #[test]
    fn lowercase_version_attribute_extracted() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <PackageReference Include="Moq" version="4.18.4" />
  </ItemGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "Moq");
        assert_eq!(deps[0].current_value, "4.18.4");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts msbuild sdk from the Sdk attr of Project element" — nuget/extract.spec.ts line 94
    #[test]
    fn msbuild_sdk_from_project_attr() {
        let content = r#"<Project Sdk="Microsoft.Build.NoTargets/3.4.0">
  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
  </PropertyGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "Microsoft.Build.NoTargets");
        assert_eq!(deps[0].current_value, "3.4.0");
        assert_eq!(deps[0].dep_type, NuGetDepType::MsbuildSdk);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "does not extract msbuild sdk from the Sdk attr of Project element if version is missing" — nuget/extract.spec.ts line 117
    #[test]
    fn msbuild_sdk_missing_version_from_project_attr() {
        let content = r#"<Project Sdk="Microsoft.Build.NoTargets">
  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
  </PropertyGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "extracts msbuild sdk from the Sdk element" — nuget/extract.spec.ts line 132
    #[test]
    fn msbuild_sdk_from_sdk_element() {
        let content = r#"<Project>
  <Sdk Name="Microsoft.Build.NoTargets" Version="3.4.0" />
  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
  </PropertyGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "Microsoft.Build.NoTargets");
        assert_eq!(deps[0].current_value, "3.4.0");
        assert_eq!(deps[0].dep_type, NuGetDepType::MsbuildSdk);
    }

    // Ported: "extracts msbuild sdk from the Import element" — nuget/extract.spec.ts line 172
    #[test]
    fn msbuild_sdk_from_import_element() {
        let content = r#"<Project>
  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
  </PropertyGroup>
  <Import Project="Sdk.props" Sdk="My.Custom.Sdk" Version="1.2.3" />
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "My.Custom.Sdk");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].dep_type, NuGetDepType::MsbuildSdk);
    }

    // Ported: "returns null for invalid csproj" — nuget/extract.spec.ts line 28
    #[test]
    fn invalid_xml_returns_error_or_empty() {
        // quick-xml may be lenient; check either error or no useful deps extracted
        let result = extract("\u{FEFF}  <?xml version=\"1.0\" encoding=\"utf-8\" ?>invalid xml>");
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    // Ported: "returns null if not xml" — nuget/extract.spec.ts line 43
    #[test]
    fn non_xml_content_returns_empty_or_error() {
        let content = "org.apache.curator:* =4.3.0\norg.apache.hadoop:*=3.1.4\n";
        // Non-XML: quick-xml returns an error or empty deps
        let result = extract(content);
        // Either errors or produces no deps (non-xml has no PackageReference elements)
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    // Ported: "does not extract msbuild sdk from the Sdk element if version is missing" — nuget/extract.spec.ts line 156
    #[test]
    fn msbuild_sdk_element_without_version_is_skipped() {
        let content = r#"<Project>
  <Sdk Name="Microsoft.Build.NoTargets" />
  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
  </PropertyGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "does not extract msbuild sdk from the Import element if version is missing" — nuget/extract.spec.ts line 196
    #[test]
    fn msbuild_import_element_without_version_is_skipped() {
        let content = r#"<Project>
  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
  </PropertyGroup>
  <Import Project="Sdk.props" Sdk="My.Custom.Sdk" />
</Project>"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "extracts ContainerBaseImage" — nuget/extract.spec.ts line 234
    #[test]
    fn extracts_container_base_image() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk.Worker">
  <PropertyGroup>
    <ContainerBaseImage>mcr.microsoft.com/dotnet/runtime:7.0.10</ContainerBaseImage>
  </PropertyGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "mcr.microsoft.com/dotnet/runtime");
        assert_eq!(deps[0].current_value, "7.0.10");
        assert_eq!(deps[0].current_digest, None);
        assert_eq!(deps[0].dep_type, NuGetDepType::ContainerImage);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts ContainerBaseImage with pinned digest" — nuget/extract.spec.ts line 260
    #[test]
    fn extracts_container_base_image_with_digest() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk.Worker">
  <PropertyGroup>
    <ContainerBaseImage>mcr.microsoft.com/dotnet/runtime:7.0.10@sha256:181067029e094856691ee1ce3782ea3bd3fda01bb5b6d19411d0f673cab1ab19</ContainerBaseImage>
  </PropertyGroup>
</Project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "mcr.microsoft.com/dotnet/runtime");
        assert_eq!(deps[0].current_value, "7.0.10");
        assert_eq!(
            deps[0].current_digest,
            Some(
                "sha256:181067029e094856691ee1ce3782ea3bd3fda01bb5b6d19411d0f673cab1ab19"
                    .to_owned()
            )
        );
        assert_eq!(deps[0].dep_type, NuGetDepType::ContainerImage);
        assert!(deps[0].skip_reason.is_none());
    }

    fn project_with_autofac() -> &'static str {
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <Version>0.1.0</Version>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Autofac" Version="4.5.0" />
  </ItemGroup>
</Project>"#
    }

    fn nuget_config_with_sources(sources: &[&str]) -> String {
        let mut config = String::from("<configuration><packageSources>");
        for (index, source) in sources.iter().enumerate() {
            config.push_str(&format!(r#"<add key="source {index}" value="{source}" />"#));
        }
        config.push_str("</packageSources></configuration>");
        config
    }

    // Ported: "considers NuGet.config" — nuget/extract.spec.ts line 289
    #[test]
    fn project_file_considers_nuget_config() {
        let config = nuget_config_with_sources(&[
            "https://api.nuget.org/v3/index.json#protocolVersion=3",
            "https://contoso.com/packages/",
        ]);
        let files = [
            (
                "with-config-file/with-config-file.csproj",
                Some(project_with_autofac()),
            ),
            ("with-config-file/NuGet.config", Some(config.as_str())),
        ];
        let package_file = extract_project_file_with_config(
            project_with_autofac(),
            "with-config-file/with-config-file.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        assert_eq!(
            package_file.deps[0].registry_urls,
            vec![
                "https://api.nuget.org/v3/index.json#protocolVersion=3",
                "https://contoso.com/packages/"
            ]
        );
        assert_eq!(package_file.package_file_version.as_deref(), Some("0.1.0"));
    }

    // Ported: "considers lower-case nuget.config" — nuget/extract.spec.ts line 309
    #[test]
    fn project_file_considers_lowercase_nuget_config() {
        let config = nuget_config_with_sources(&[
            "https://api.nuget.org/v3/index.json#protocolVersion=3",
            "https://contoso.com/packages/",
        ]);
        let files = [
            (
                "with-lower-case-config-file/with-lower-case-config-file.csproj",
                Some(project_with_autofac()),
            ),
            (
                "with-lower-case-config-file/nuget.config",
                Some(config.as_str()),
            ),
        ];
        let package_file = extract_project_file_with_config(
            project_with_autofac(),
            "with-lower-case-config-file/with-lower-case-config-file.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        assert_eq!(
            package_file.deps[0].registry_urls,
            vec![
                "https://api.nuget.org/v3/index.json#protocolVersion=3",
                "https://contoso.com/packages/"
            ]
        );
    }

    // Ported: "considers pascal-case NuGet.Config" — nuget/extract.spec.ts line 330
    #[test]
    fn project_file_considers_pascal_case_nuget_config() {
        let config = nuget_config_with_sources(&[
            "https://api.nuget.org/v3/index.json#protocolVersion=3",
            "https://contoso.com/packages/",
        ]);
        let files = [
            (
                "with-pascal-case-config-file/with-pascal-case-config-file.csproj",
                Some(project_with_autofac()),
            ),
            (
                "with-pascal-case-config-file/NuGet.Config",
                Some(config.as_str()),
            ),
        ];
        let package_file = extract_project_file_with_config(
            project_with_autofac(),
            "with-pascal-case-config-file/with-pascal-case-config-file.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        assert_eq!(
            package_file.deps[0].registry_urls,
            vec![
                "https://api.nuget.org/v3/index.json#protocolVersion=3",
                "https://contoso.com/packages/"
            ]
        );
    }

    // Ported: "handles malformed NuGet.config" — nuget/extract.spec.ts line 351
    #[test]
    fn project_file_ignores_malformed_nuget_config() {
        let files = [
            (
                "with-malformed-config-file/with-malformed-config-file.csproj",
                Some(project_with_autofac()),
            ),
            (
                "with-malformed-config-file/NuGet.config",
                Some("<<< not xml >>>"),
            ),
        ];
        let package_file = extract_project_file_with_config(
            project_with_autofac(),
            "with-malformed-config-file/with-malformed-config-file.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        assert!(package_file.deps[0].registry_urls.is_empty());
    }

    // Ported: "handles NuGet.config without package sources" — nuget/extract.spec.ts line 368
    #[test]
    fn project_file_ignores_nuget_config_without_package_sources() {
        let files = [
            (
                "without-package-sources/without-package-sources.csproj",
                Some(project_with_autofac()),
            ),
            (
                "without-package-sources/NuGet.config",
                Some("<configuration></configuration>"),
            ),
        ];
        let package_file = extract_project_file_with_config(
            project_with_autofac(),
            "without-package-sources/without-package-sources.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        assert!(package_file.deps[0].registry_urls.is_empty());
    }

    // Ported: "handles NuGet.config with whitespaces in package source keys" — nuget/extract.spec.ts line 385
    #[test]
    fn project_file_handles_whitespace_package_source_keys() {
        let project = r#"<Project>
  <ItemGroup>
    <PackageReference Include="Newtonsoft.Json" Version="12.0.3" />
  </ItemGroup>
</Project>"#;
        let config = r#"<configuration>
  <packageSources>
    <add key=" nuget.org " value="https://api.nuget.org/v3/index.json#protocolVersion=3" />
    <add key=" my get " value="https://my.myget.org/F/my/auth/guid/api/v3/index.json" />
  </packageSources>
</configuration>"#;
        let files = [
            ("with-whitespaces/with-whitespaces.csproj", Some(project)),
            ("with-whitespaces/NuGet.config", Some(config)),
        ];
        let package_file = extract_project_file_with_config(
            project,
            "with-whitespaces/with-whitespaces.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        assert_eq!(package_file.deps[0].package_id, "Newtonsoft.Json");
        assert_eq!(
            package_file.deps[0].registry_urls,
            vec![
                "https://api.nuget.org/v3/index.json#protocolVersion=3",
                "https://my.myget.org/F/my/auth/guid/api/v3/index.json"
            ]
        );
    }

    // Ported: "ignores local feed in NuGet.config" — nuget/extract.spec.ts line 404
    #[test]
    fn project_file_ignores_local_feed_in_nuget_config() {
        let config =
            nuget_config_with_sources(&[r#"C:\local\packages"#, "https://contoso.com/packages/"]);
        let files = [
            (
                "with-local-feed-in-config-file/with-local-feed-in-config-file.csproj",
                Some(project_with_autofac()),
            ),
            (
                "with-local-feed-in-config-file/NuGet.config",
                Some(config.as_str()),
            ),
        ];
        let package_file = extract_project_file_with_config(
            project_with_autofac(),
            "with-local-feed-in-config-file/with-local-feed-in-config-file.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        assert_eq!(
            package_file.deps[0].registry_urls,
            vec!["https://contoso.com/packages/"]
        );
    }

    // Ported: "extracts registry URLs independently" — nuget/extract.spec.ts line 422
    #[test]
    fn project_files_extract_registry_urls_independently() {
        let one_config = nuget_config_with_sources(&["https://api.nuget.org/v3/index.json"]);
        let two_config = nuget_config_with_sources(&["https://contoso.com/packages/"]);
        let files = [
            (
                "multiple-package-files/one/one.csproj",
                Some(project_with_autofac()),
            ),
            (
                "multiple-package-files/one/NuGet.config",
                Some(one_config.as_str()),
            ),
            (
                "multiple-package-files/two/two.csproj",
                Some(project_with_autofac()),
            ),
            (
                "multiple-package-files/two/NuGet.config",
                Some(two_config.as_str()),
            ),
        ];
        let one = extract_project_file_with_config(
            project_with_autofac(),
            "multiple-package-files/one/one.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");
        let two = extract_project_file_with_config(
            project_with_autofac(),
            "multiple-package-files/two/two.csproj",
            false,
            &files,
        )
        .expect("parse should succeed")
        .expect("deps should be extracted");

        assert_eq!(
            one.deps[0].registry_urls,
            vec!["https://api.nuget.org/v3/index.json"]
        );
        assert_eq!(
            two.deps[0].registry_urls,
            vec!["https://contoso.com/packages/"]
        );
    }

    // Ported: "works" — nuget/extract.spec.ts line 521
    #[test]
    fn dotnet_tools_manifest_extracts_tools() {
        let content = r#"{
  "version": 1,
  "isRoot": true,
  "tools": {
    "minver-cli": {
      "version": "2.0.0",
      "commands": ["minver"]
    }
  }
}"#;
        let deps = extract_dotnet_tools(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_id, "minver-cli");
        assert_eq!(deps[0].current_value, "2.0.0");
        assert_eq!(deps[0].dep_type, NuGetDepType::DotnetTool);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "with-config" — nuget/extract.spec.ts line 537
    #[test]
    fn dotnet_tools_manifest_applies_parent_nuget_config() {
        let content = r#"{
  "version": 1,
  "isRoot": true,
  "tools": {
    "minver-cli": {
      "version": "2.0.0",
      "commands": ["minver"]
    }
  }
}"#;
        let config = nuget_config_with_sources(&[
            "https://api.nuget.org/v3/index.json#protocolVersion=3",
            "https://contoso.com/packages/",
        ]);
        let files = [
            ("with-config-file/.config/dotnet-tools.json", Some(content)),
            ("with-config-file/NuGet.config", Some(config.as_str())),
        ];
        let deps = extract_dotnet_tools_with_config(
            content,
            "with-config-file/.config/dotnet-tools.json",
            &files,
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].registry_urls,
            vec![
                "https://api.nuget.org/v3/index.json#protocolVersion=3",
                "https://contoso.com/packages/"
            ]
        );
    }

    // Ported: "wrong version" — nuget/extract.spec.ts line 561
    #[test]
    fn dotnet_tools_manifest_wrong_version_returns_empty() {
        let deps = extract_dotnet_tools(
            r#"{"version": 2, "tools": {"minver-cli": {"version": "2.0.0"}}}"#,
        );
        assert!(deps.is_empty());
    }

    // Ported: "returns null for no deps" — nuget/extract.spec.ts line 571
    #[test]
    fn dotnet_tools_manifest_without_tools_returns_empty() {
        let deps = extract_dotnet_tools(r#"{"version": 1}"#);
        assert!(deps.is_empty());
    }

    // Ported: "does not throw" — nuget/extract.spec.ts line 577
    #[test]
    fn dotnet_tools_manifest_malformed_returns_empty() {
        let deps = extract_dotnet_tools("{{");
        assert!(deps.is_empty());
    }

    // Ported: "reads sdk and package directives" — nuget/extract.spec.ts line 583
    #[test]
    fn single_csharp_file_reads_sdk_and_package_directives() {
        let content = r#"
          #:sdk Some.Sdk@6.0.0
          #:package Some.NuGet.Package@3.0.1

          Console.WriteLine("Hello World!");
        "#;
        let deps = extract_single_csharp_file(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].package_id, "Some.Sdk");
        assert_eq!(deps[0].current_value, "6.0.0");
        assert_eq!(deps[0].dep_type, NuGetDepType::MsbuildSdk);
        assert_eq!(deps[0].dep_type.as_renovate_str(), "msbuild-sdk");
        assert_eq!(deps[1].package_id, "Some.NuGet.Package");
        assert_eq!(deps[1].current_value, "3.0.1");
        assert_eq!(deps[1].dep_type, NuGetDepType::SingleFilePackage);
        assert_eq!(deps[1].dep_type.as_renovate_str(), "nuget");
    }

    // Ported: "calls applyRegistries to honor nuget.config files if present" — nuget/extract.spec.ts line 615
    #[test]
    fn single_csharp_file_applies_nuget_config_registries() {
        let content = r#"
#:sdk Some.Sdk@6.0.0
#:package Some.NuGet.Package@3.0.1

Console.WriteLine("Hello World!");
"#;
        let config = nuget_config_with_sources(&["https://contoso.com/packages/"]);
        let files = [
            ("single-csharp-file-nuget/singlefile.cs", Some(content)),
            (
                "single-csharp-file-nuget/NuGet.config",
                Some(config.as_str()),
            ),
        ];
        let deps = extract_single_csharp_file_with_config(
            content,
            "single-csharp-file-nuget/singlefile.cs",
            &files,
        );
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].registry_urls, vec!["https://contoso.com/packages/"]);
        assert_eq!(deps[1].registry_urls, vec!["https://contoso.com/packages/"]);
    }

    // Ported: "extracts msbuild-sdks from global.json" — nuget/extract.spec.ts line 461
    #[test]
    fn global_json_extracts_dotnet_sdk_and_msbuild_sdks() {
        let content = r#"{
  "sdk": {
    "version": "5.0.302",
    "rollForward": "latestMajor"
  },
  "msbuild-sdks": {
    "YoloDev.Sdk": "0.2.0"
  }
}"#;
        let extracted = extract_global_json(content).expect("global.json should extract");
        assert_eq!(extracted.dotnet_sdk_constraint, Some("5.0.302".to_owned()));
        assert_eq!(extracted.deps.len(), 2);
        assert_eq!(extracted.deps[0].package_id, "dotnet-sdk");
        assert_eq!(extracted.deps[0].current_value, "5.0.302");
        assert_eq!(extracted.deps[0].dep_type, NuGetDepType::DotnetSdk);
        assert_eq!(extracted.deps[0].dep_type.as_renovate_str(), "dotnet-sdk");
        assert_eq!(extracted.deps[1].package_id, "YoloDev.Sdk");
        assert_eq!(extracted.deps[1].current_value, "0.2.0");
        assert_eq!(extracted.deps[1].dep_type, NuGetDepType::MsbuildSdk);
    }

    // Ported: "extracts dotnet-sdk from global.json" — nuget/extract.spec.ts line 483
    #[test]
    fn global_json_extracts_dotnet_sdk_only() {
        let content = r#"{
  "sdk": {
    "version": "5.0.302",
    "rollForward": "latestMajor"
  }
}"#;
        let extracted = extract_global_json(content).expect("global.json should extract");
        assert_eq!(extracted.dotnet_sdk_constraint, Some("5.0.302".to_owned()));
        assert_eq!(extracted.deps.len(), 1);
        assert_eq!(extracted.deps[0].package_id, "dotnet-sdk");
        assert_eq!(extracted.deps[0].current_value, "5.0.302");
        assert_eq!(extracted.deps[0].dep_type, NuGetDepType::DotnetSdk);
    }

    // Ported: "handles malformed global.json" — nuget/extract.spec.ts line 501
    #[test]
    fn global_json_malformed_returns_none() {
        assert!(extract_global_json("{{").is_none());
    }

    // Ported: "handles not-a-nuget global.json" — nuget/extract.spec.ts line 509
    #[test]
    fn global_json_without_nuget_content_returns_none() {
        assert!(extract_global_json(r#"{"sdk": {"rollForward": "latestMajor"}}"#).is_none());
    }
}

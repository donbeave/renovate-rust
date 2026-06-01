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

// ── .csproj version bumping ───────────────────────────────────────────────────

/// Bump the `<Version>` or `<VersionPrefix>` element in a .csproj XML file.
///
/// Returns the updated content. If bumping fails (invalid semver, version not
/// found, unknown bump type), the original content is returned unchanged.
///
/// Mirrors `lib/modules/manager/nuget/update.ts` `bumpPackageVersion()`.
pub fn bump_package_version(content: &str, current_value: &str, bump_version: &str) -> String {
    let Ok(current) = semver::Version::parse(current_value) else {
        return content.to_owned();
    };
    let Some(new_version) = bump_csproj_semver(&current, bump_version) else {
        return content.to_owned();
    };

    for tag in &["Version", "VersionPrefix"] {
        let old_tag = format!("<{tag}>{current_value}</{tag}>");
        let new_tag = format!("<{tag}>{new_version}</{tag}>");
        if content.contains(&old_tag) {
            return content.replacen(&old_tag, &new_tag, 1);
        }
    }
    content.to_owned()
}

/// Find the `<Version>` or `<VersionPrefix>` element value in a .csproj XML string.
///
/// Mirrors `findVersion()` from `lib/modules/manager/nuget/util.ts`.
pub fn find_version_in_csproj(content: &str) -> Option<String> {
    let mut reader = Reader::from_reader(BufReader::new(content.as_bytes()));
    let mut buf = Vec::new();
    let mut in_property_group = false;
    let mut current_tag = String::new();
    let mut version: Option<String> = None;
    let mut version_prefix: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if name == "PropertyGroup" {
                    in_property_group = true;
                } else if in_property_group && (name == "Version" || name == "VersionPrefix") {
                    current_tag = name;
                }
            }
            Ok(Event::Text(e)) => {
                if !current_tag.is_empty() {
                    let val = e.decode().ok()?.into_owned();
                    if current_tag == "Version" {
                        version = Some(val);
                    } else if current_tag == "VersionPrefix" && version.is_none() {
                        version_prefix = Some(val);
                    }
                    current_tag.clear();
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if name == "PropertyGroup" {
                    in_property_group = false;
                }
                current_tag.clear();
            }
            Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if in_property_group && (name == "Version" || name == "VersionPrefix") {
                    // empty tag like <Version /> — no value
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    version.or(version_prefix)
}

/// Parse a NuGet.config XML string and return the list of active registries.
///
/// Mirrors `getConfiguredRegistries()` from `lib/modules/manager/nuget/util.ts`.
pub fn parse_nuget_config_registries_full(content: &str) -> Vec<NuGetRegistry> {
    let mut reader = Reader::from_reader(BufReader::new(content.as_bytes()));
    let mut buf = Vec::new();

    // Check if packageSources exists; if not, return empty (mirrors upstream returning undefined)
    let mut has_package_sources = false;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if name == "packageSources" {
                    has_package_sources = true;
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    if !has_package_sources {
        return Vec::new();
    }

    // Parse source mapping, disabled sources, and registry adds in a single pass
    let mut reader = Reader::from_reader(BufReader::new(content.as_bytes()));
    let mut buf = Vec::new();
    let mut registries: Vec<NuGetRegistry> = vec![NuGetRegistry {
        name: Some("nuget.org".to_owned()),
        url: "https://api.nuget.org/v3/index.json".to_owned(),
        source_mapped_package_patterns: None,
    }];
    let mut disabled_sources: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut in_package_sources = false;
    let mut in_disabled_sources = false;
    let mut source_mapping: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut in_package_source_mapping = false;
    let mut current_mapping_key = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "packageSources" => in_package_sources = true,
                    "disabledPackageSources" => in_disabled_sources = true,
                    "packageSourceMapping" => in_package_source_mapping = true,
                    "clear" => {
                        if in_package_sources {
                            registries.clear();
                        }
                    }
                    "add" => {
                        if in_package_sources {
                            let mut key = String::new();
                            let mut value = String::new();
                            let mut protocol_version = String::new();
                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    let attr_name = String::from_utf8_lossy(a.key.as_ref()).into_owned();
                                    if let Ok(val) = a.unescape_value() {
                                        match attr_name.as_str() {
                                            "key" => key = val.into_owned(),
                                            "value" => value = val.into_owned(),
                                            "protocolVersion" => protocol_version = val.into_owned(),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            if value.starts_with("http://") || value.starts_with("https://") {
                                let mut url = value;
                                if !protocol_version.is_empty() {
                                    url = format!("{}#protocolVersion={}", url, protocol_version);
                                }
                                let patterns = source_mapping.get(&key).cloned();
                                registries.push(NuGetRegistry {
                                    name: Some(key),
                                    url,
                                    source_mapped_package_patterns: patterns,
                                });
                            }
                        }
                        if in_disabled_sources {
                            let mut key = String::new();
                            let mut value = String::new();
                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    let attr_name = String::from_utf8_lossy(a.key.as_ref()).into_owned();
                                    if let Ok(val) = a.unescape_value() {
                                        match attr_name.as_str() {
                                            "key" => key = val.into_owned(),
                                            "value" => value = val.into_owned(),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            if value == "true" {
                                disabled_sources.insert(key);
                            }
                        }
                    }
                    "packageSource" => {
                        if in_package_source_mapping {
                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    if String::from_utf8_lossy(a.key.as_ref()) == "key" {
                                        if let Ok(val) = a.unescape_value() {
                                            current_mapping_key = val.into_owned();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "package" => {
                        if in_package_source_mapping {
                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    if String::from_utf8_lossy(a.key.as_ref()) == "pattern" {
                                        if let Ok(val) = a.unescape_value() {
                                            source_mapping
                                                .entry(current_mapping_key.clone())
                                                .or_default()
                                                .push(val.into_owned());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "packageSources" => in_package_sources = false,
                    "disabledPackageSources" => in_disabled_sources = false,
                    "packageSourceMapping" => in_package_source_mapping = false,
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }

    // Apply disabled sources
    if !disabled_sources.is_empty() {
        registries.retain(|r| r.name.as_ref().map_or(true, |n| !disabled_sources.contains(n)));
    }

    // Apply source mapping to default registries
    for reg in &mut registries {
        if reg.name.as_deref() == Some("nuget.org") && reg.source_mapped_package_patterns.is_none() {
            if let Some(patterns) = source_mapping.get("nuget.org") {
                reg.source_mapped_package_patterns = Some(patterns.clone());
            }
        }
    }

    // Deduplicate registries with #protocolVersion=3
    // Keep any which include sourceMappedPackagePatterns
    let plain_urls: std::collections::HashSet<String> = registries
        .iter()
        .filter(|r| r.source_mapped_package_patterns.is_none())
        .map(|r| r.url.clone())
        .collect();
    registries.retain(|r| {
        if r.source_mapped_package_patterns.is_some() {
            return true;
        }
        let alt = format!("{}#protocolVersion=3", r.url);
        !plain_urls.contains(&alt)
    });

    registries
}

/// Minimal dependency representation for util-layer registry mapping.
#[derive(Debug, Clone, Default)]
pub struct NuGetPackageDependency {
    pub dep_name: String,
    pub registry_urls: Option<Vec<String>>,
}

/// Apply registry URLs to a dependency, respecting source mapping patterns.
///
/// Mirrors `applyRegistries()` from `lib/modules/manager/nuget/util.ts`.
pub fn apply_registries_to_dep(dep: &mut NuGetPackageDependency, registries: &[NuGetRegistry]) {
    if registries.is_empty() {
        return;
    }

    let has_source_mapping = registries.iter().any(|r| r.source_mapped_package_patterns.is_some());
    if !has_source_mapping {
        dep.registry_urls = Some(registries.iter().map(|r| r.url.clone()).collect());
        return;
    }

    let dep_name = dep.dep_name.as_str();
    let mut matched_urls: Vec<String> = Vec::new();
    let mut patterns: Vec<(String, Vec<String>)> = Vec::new();

    for reg in registries {
        if let Some(ref pats) = reg.source_mapped_package_patterns {
            for pat in pats {
                patterns.push((pat.clone(), vec![reg.url.clone()]));
            }
        }
    }

    // Sort patterns: exact matches first, then wildcards, longest first
    patterns.sort_by(|a, b| {
        let a_wild = a.0.ends_with('*');
        let b_wild = b.0.ends_with('*');
        if a_wild && !b_wild {
            return std::cmp::Ordering::Greater;
        }
        if !a_wild && b_wild {
            return std::cmp::Ordering::Less;
        }
        b.0.len().cmp(&a.0.len())
    });

    for (pattern, urls) in &patterns {
        let glob = pattern.trim_end_matches('*');
        if pattern == "*" || dep_name.eq_ignore_ascii_case(glob) || dep_name.to_ascii_lowercase().starts_with(&glob.to_ascii_lowercase()) {
            matched_urls.extend(urls.iter().cloned());
            break;
        }
    }

    if !matched_urls.is_empty() {
        dep.registry_urls = Some(matched_urls);
    }
}

// ── Package-tree helpers ──────────────────────────────────────────────────

/// A project file in the dependency tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectFile {
    pub name: String,
    pub is_leaf: bool,
}

const GLOBAL_JSON: &str = "global.json";
const NUGET_CENTRAL_FILE: &str = "Directory.Packages.props";
const MSBUILD_CENTRAL_FILE: &str = "Packages.props";
const DIRECTORY_BUILD_PROPS: &str = "Directory.Build.props";

/// Pure, testable version of `getDependentPackageFiles`.
///
/// Mirrors `getDependentPackageFiles()` from `lib/modules/manager/nuget/package-tree.ts`.
pub fn get_dependent_package_files_pure(
    package_file_name: &str,
    file_list: &[String],
    file_contents: &std::collections::HashMap<String, String>,
    is_props_file: bool,
    is_global_json: bool,
) -> Result<Vec<ProjectFile>, String> {
    let package_files = get_all_package_files(file_list);
    let mut graph: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    if is_props_file {
        graph.entry(package_file_name.to_owned()).or_default();
    }
    if is_global_json {
        graph.entry(GLOBAL_JSON.to_owned()).or_default();
    }

    let parent_dir = if package_file_name == NUGET_CENTRAL_FILE
        || package_file_name == MSBUILD_CENTRAL_FILE
        || package_file_name == GLOBAL_JSON
        || package_file_name == DIRECTORY_BUILD_PROPS
    {
        ""
    } else {
        std::path::Path::new(package_file_name)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
    };

    for f in &package_files {
        graph.entry(f.clone()).or_default();
        if (is_props_file || is_global_json)
            && std::path::Path::new(f)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("")
                .starts_with(parent_dir)
        {
            graph
                .entry(package_file_name.to_owned())
                .or_default()
                .push(f.clone());
        }
    }

    for f in &package_files {
        let Some(content) = file_contents.get(f) else { continue };
        let refs = extract_project_references(content);
        for r in refs {
            let normalized = reframe_relative_path_to_root_of_repo(f, &r);
            graph.entry(f.clone()).or_default().push(normalized);
        }
        if has_cycle(&graph) {
            return Err("Circular reference detected in NuGet package files".to_owned());
        }
    }

    let mut deps: Vec<(String, bool)> = Vec::new();
    recursively_get_dependent_package_files(package_file_name, &graph, &mut deps);

    if is_props_file || is_global_json {
        deps.retain(|(name, _)| name != package_file_name);
    }

    let result: Vec<ProjectFile> = deps
        .into_iter()
        .map(|(name, is_leaf)| ProjectFile { name, is_leaf })
        .collect();
    Ok(result)
}

fn get_all_package_files(file_list: &[String]) -> Vec<String> {
    file_list
        .iter()
        .filter(|f| {
            let lower = f.to_lowercase();
            lower.ends_with(".csproj")
                || lower.ends_with(".vbproj")
                || lower.ends_with(".fsproj")
        })
        .cloned()
        .collect()
}

fn extract_project_references(content: &str) -> Vec<String> {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    let mut reader = Reader::from_reader(std::io::BufReader::new(content.as_bytes()));
    let mut buf = Vec::new();
    let mut in_item_group = false;
    let mut refs = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if name == "ItemGroup" {
                    in_item_group = true;
                } else if in_item_group && name == "ProjectReference" {
                    for attr in e.attributes() {
                        if let Ok(a) = attr {
                            if String::from_utf8_lossy(a.key.as_ref()) == "Include" {
                                if let Ok(val) = a.unescape_value() {
                                    let v = val.into_owned();
                                    if !v.is_empty() {
                                        refs.push(v);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if name == "ItemGroup" {
                    in_item_group = false;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    refs
}

fn normalize_relative_path(path: &str) -> String {
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            ".." => {
                parts.pop();
            }
            "." | "" => {}
            _ => parts.push(part),
        }
    }
    parts.join("/")
}

fn reframe_relative_path_to_root_of_repo(
    dependent_project_relative_path: &str,
    project_reference: &str,
) -> String {
    let dir = std::path::Path::new(dependent_project_relative_path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("");
    let resolved = if dir.is_empty() {
        project_reference.to_owned()
    } else {
        format!("{}/{}", dir, project_reference)
    };
    normalize_relative_path(&resolved)
}

fn has_cycle(graph: &std::collections::HashMap<String, Vec<String>>) -> bool {
    let mut visited = std::collections::HashSet::new();
    let mut stack = std::collections::HashSet::new();

    for node in graph.keys() {
        if !visited.contains(node) {
            if dfs_cycle(node, graph, &mut visited, &mut stack) {
                return true;
            }
        }
    }
    false
}

fn dfs_cycle(
    node: &str,
    graph: &std::collections::HashMap<String, Vec<String>>,
    visited: &mut std::collections::HashSet<String>,
    stack: &mut std::collections::HashSet<String>,
) -> bool {
    visited.insert(node.to_owned());
    stack.insert(node.to_owned());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                if dfs_cycle(neighbor, graph, visited, stack) {
                    return true;
                }
            } else if stack.contains(neighbor) {
                return true;
            }
        }
    }

    stack.remove(node);
    false
}

fn recursively_get_dependent_package_files(
    package_file_name: &str,
    graph: &std::collections::HashMap<String, Vec<String>>,
    deps: &mut Vec<(String, bool)>,
) {
    if deps.iter().any(|(n, _)| n == package_file_name) {
        return;
    }

    let dependents = graph.get(package_file_name).cloned().unwrap_or_default();

    if dependents.is_empty() {
        deps.push((package_file_name.to_owned(), true));
        return;
    }

    deps.push((package_file_name.to_owned(), false));

    for dep in dependents {
        recursively_get_dependent_package_files(&dep, graph, deps);
    }
}

fn bump_csproj_semver(version: &semver::Version, bump_type: &str) -> Option<String> {
    let mut new = version.clone();
    match bump_type {
        "patch" => {
            new.patch += 1;
            new.pre = semver::Prerelease::EMPTY;
        }
        "minor" => {
            new.minor += 1;
            new.patch = 0;
            new.pre = semver::Prerelease::EMPTY;
        }
        "major" => {
            new.major += 1;
            new.minor = 0;
            new.patch = 0;
            new.pre = semver::Prerelease::EMPTY;
        }
        "prerelease" => {
            let pre_str = new.pre.as_str();
            if pre_str.is_empty() {
                new.pre = semver::Prerelease::new("1").ok()?;
            } else if let Ok(n) = pre_str.parse::<u64>() {
                new.pre = semver::Prerelease::new(&(n + 1).to_string()).ok()?;
            } else {
                return None;
            }
        }
        _ => return None,
    }
    Some(new.to_string())
}

// ---------------------------------------------------------------------------
// NuGet config formatter — lib/modules/manager/nuget/config-formatter.ts
// ---------------------------------------------------------------------------

/// A NuGet package source registry entry.
#[derive(Debug, Clone)]
pub struct NuGetRegistry {
    pub name: Option<String>,
    pub url: String,
    pub source_mapped_package_patterns: Option<Vec<String>>,
}

/// Parsed NuGet registry URL with feed URL and protocol version.
#[derive(Debug, Clone)]
pub struct ParsedNuGetRegistryUrl {
    pub feed_url: String,
    pub protocol_version: u8,
}

/// Parse a NuGet registry URL, extracting protocol version from hash or path.
///
/// Mirrors `parseRegistryUrl` from `lib/modules/datasource/nuget/common.ts`.
pub fn parse_nuget_registry_url(registry_url: &str) -> ParsedNuGetRegistryUrl {
    let Ok(mut parsed) = url::Url::parse(registry_url) else {
        return ParsedNuGetRegistryUrl {
            feed_url: registry_url.to_owned(),
            protocol_version: 2,
        };
    };

    let protocol_version;
    if let Some(frag) = parsed.fragment() {
        if let Some(proto) = frag.strip_prefix("protocolVersion=") {
            protocol_version = proto.parse::<u8>().unwrap_or(2);
            parsed.set_fragment(None);
        } else {
            protocol_version = if parsed.path().ends_with(".json") {
                3
            } else {
                2
            };
        }
    } else {
        protocol_version = if parsed.path().ends_with(".json") {
            3
        } else {
            2
        };
    }

    ParsedNuGetRegistryUrl {
        feed_url: parsed.to_string(),
        protocol_version,
    }
}

/// Escape a registry name for use as an XML element name.
///
/// Characters that are not alphanumeric, `-`, `_`, or `.` are encoded as
/// `_x{hex}_` (e.g., space → `_x0020_`).
fn escape_nuget_name(name: &str) -> String {
    let mut escaped = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.' {
            escaped.push(ch);
        } else {
            let code = ch as u32;
            escaped.push_str(&format!("_x{code:04x}_"));
        }
    }
    escaped
}

/// Generate a NuGet.Config XML from a list of registry definitions.
///
/// Mirrors `createNuGetConfigXml` from
/// `lib/modules/manager/nuget/config-formatter.ts`.
pub fn create_nuget_config_xml(registries: &[NuGetRegistry]) -> String {
    use crate::util::host_rules::{HostRuleSearch, find};
    use std::collections::HashSet;

    let mut contents = String::from(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<configuration>\n<packageSources>\n",
    );
    let mut unnamed_count: u32 = 0;
    let mut seen_urls: HashSet<String> = HashSet::new();

    struct Credential {
        name: String,
        username: Option<String>,
        password: Option<String>,
    }
    struct PackageSourceMap {
        name: String,
        patterns: Vec<String>,
    }

    let mut credentials: Vec<Credential> = Vec::new();
    let mut source_maps: Vec<PackageSourceMap> = Vec::new();

    for reg in registries {
        if seen_urls.contains(&reg.url) {
            continue;
        }
        seen_urls.insert(reg.url.clone());

        let registry_name = reg.name.clone().unwrap_or_else(|| {
            unnamed_count += 1;
            format!("Package source {unnamed_count}")
        });
        let parsed = parse_nuget_registry_url(&reg.url);

        contents.push_str(&format!(
            "<add key=\"{}\" value=\"{}\" protocolVersion=\"{}\" />\n",
            registry_name, parsed.feed_url, parsed.protocol_version
        ));

        let rule = find(&HostRuleSearch {
            host_type: Some("nuget".to_owned()),
            url: Some(reg.url.clone()),
            ..Default::default()
        });

        if rule.username.is_some() || rule.password.is_some() {
            credentials.push(Credential {
                name: registry_name.clone(),
                username: rule.username,
                password: rule.password,
            });
        }

        if let Some(ref patterns) = reg.source_mapped_package_patterns {
            source_maps.push(PackageSourceMap {
                name: registry_name,
                patterns: patterns.clone(),
            });
        }
    }

    contents.push_str("</packageSources>\n");

    if !credentials.is_empty() {
        contents.push_str("<packageSourceCredentials>\n");
        for cred in &credentials {
            let escaped = escape_nuget_name(&cred.name);
            contents.push_str(&format!("<{escaped}>\n"));
            if let Some(ref u) = cred.username {
                contents.push_str(&format!("<add key=\"Username\" value=\"{u}\" />\n"));
            }
            if let Some(ref p) = cred.password {
                contents.push_str(&format!(
                    "<add key=\"ClearTextPassword\" value=\"{p}\" />\n"
                ));
            }
            contents.push_str("<add key=\"ValidAuthenticationTypes\" value=\"basic\" />");
            contents.push_str(&format!("</{escaped}>\n"));
        }
        contents.push_str("</packageSourceCredentials>\n");
    }

    if !source_maps.is_empty() {
        contents.push_str("<packageSourceMapping>\n");
        for sm in &source_maps {
            contents.push_str(&format!("<packageSource key=\"{}\">\n", sm.name));
            for pattern in &sm.patterns {
                contents.push_str(&format!("<package pattern=\"{pattern}\" />\n"));
            }
            contents.push_str("</packageSource>\n");
        }
        contents.push_str("</packageSourceMapping>");
    }

    contents.push_str("</configuration>\n");
    contents
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

    // ── nuget bump_package_version tests ─────────────────────────────────────

    const SIMPLE_CONTENT: &str = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><Version>0.0.1</Version></PropertyGroup></Project>"#;
    const MINIMUM_CONTENT: &str = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><Version>1</Version></PropertyGroup></Project>"#;
    const PRERELEASE_CONTENT: &str = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><Version>1.0.0-1</Version></PropertyGroup></Project>"#;
    const ISSUE23526_INITIAL: &str = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><Version>4.9.0</Version></PropertyGroup></Project>"#;
    const ISSUE23526_EXPECTED: &str = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><Version>4.10.0</Version></PropertyGroup></Project>"#;

    // Ported: "bumps csproj version" — modules/manager/nuget/update.spec.ts line 17
    #[test]
    fn nuget_bumps_csproj_version() {
        let result = bump_package_version(SIMPLE_CONTENT, "0.0.1", "patch");
        assert!(result.contains("<Version>0.0.2</Version>"));
    }

    // Ported: "does not bump version twice" — modules/manager/nuget/update.spec.ts line 28
    #[test]
    fn nuget_does_not_bump_twice() {
        let bumped = bump_package_version(SIMPLE_CONTENT, "0.0.1", "patch");
        let bumped2 = bump_package_version(&bumped, "0.0.1", "patch");
        assert_eq!(bumped, bumped2);
    }

    // Ported: "issue 23526 does not bump version incorrectly" — modules/manager/nuget/update.spec.ts line 43
    #[test]
    fn nuget_issue_23526_minor_bump() {
        let bumped = bump_package_version(ISSUE23526_INITIAL, "4.9.0", "minor");
        let bumped2 = bump_package_version(&bumped, "4.9.0", "minor");
        assert_eq!(bumped2, ISSUE23526_EXPECTED);
    }

    // Ported: "does not bump version if version is not a semantic version" — modules/manager/nuget/update.spec.ts line 58
    #[test]
    fn nuget_does_not_bump_non_semver() {
        let result = bump_package_version(MINIMUM_CONTENT, "1", "patch");
        assert!(result.contains("<Version>1</Version>"));
        assert!(!result.contains("<Version>2</Version>"));
    }

    // Ported: "does not bump version if extract found no version" — modules/manager/nuget/update.spec.ts line 69
    #[test]
    fn nuget_does_not_bump_empty_current_value() {
        let result = bump_package_version(MINIMUM_CONTENT, "", "patch");
        assert_eq!(result, MINIMUM_CONTENT);
    }

    // Ported: "does not bump version if csproj has no version" — modules/manager/nuget/update.spec.ts line 75
    #[test]
    fn nuget_does_not_bump_when_no_version_tag() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#;
        let result = bump_package_version(content, "0.0.1", "patch");
        assert_eq!(result, content);
    }

    // Ported: "returns content if bumping errors" — modules/manager/nuget/update.spec.ts line 87
    #[test]
    fn nuget_returns_content_on_invalid_bump_type() {
        let result = bump_package_version(SIMPLE_CONTENT, "0.0.1", "not_a_bump_type");
        assert_eq!(result, SIMPLE_CONTENT);
    }

    // Ported: "bumps csproj version with prerelease semver level" — modules/manager/nuget/update.spec.ts line 96
    #[test]
    fn nuget_bumps_prerelease_version() {
        let result = bump_package_version(PRERELEASE_CONTENT, "1.0.0-1", "prerelease");
        assert!(result.contains("<Version>1.0.0-2</Version>"));
    }

    // Ported: "bumps csproj version prefix" — modules/manager/nuget/update.spec.ts line 107
    #[test]
    fn nuget_bumps_version_prefix() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><VersionPrefix>1.0.0</VersionPrefix></PropertyGroup></Project>"#;
        let result = bump_package_version(content, "1.0.0", "patch");
        assert!(result.contains("<VersionPrefix>1.0.1</VersionPrefix>"));
    }

    // ── createNuGetConfigXml ─────────────────────────────────────────────────

    // Ported: "returns xml with registries" — manager/nuget/config-formatter.spec.ts line 12
    #[test]
    fn nuget_config_xml_basic_registries() {
        crate::util::host_rules::clear();
        let xml = create_nuget_config_xml(&[
            NuGetRegistry {
                name: Some("myRegistry".to_owned()),
                url: "https://my-registry.example.org".to_owned(),
                source_mapped_package_patterns: None,
            },
            NuGetRegistry {
                name: Some("myRegistry2".to_owned()),
                url: "https://my-registry2.example.org/index.json".to_owned(),
                source_mapped_package_patterns: None,
            },
            NuGetRegistry {
                name: None,
                url: "https://my-unnamed-registry.example.org/index.json".to_owned(),
                source_mapped_package_patterns: None,
            },
        ]);
        assert!(xml.contains(
            r#"key="myRegistry" value="https://my-registry.example.org/" protocolVersion="2""#
        ));
        assert!(xml.contains(r#"key="myRegistry2" value="https://my-registry2.example.org/index.json" protocolVersion="3""#));
        assert!(xml.contains(r#"key="Package source 1""#));
        assert!(!xml.contains("packageSourceCredentials"));
        assert!(!xml.contains("packageSourceMapping"));
    }

    // Ported: "strips protocol version from feed url" — manager/nuget/config-formatter.spec.ts line 181
    #[test]
    fn nuget_config_xml_strips_protocol_version_from_hash() {
        crate::util::host_rules::clear();
        let xml = create_nuget_config_xml(&[NuGetRegistry {
            name: Some("myRegistry".to_owned()),
            url: "https://my-registry.example.org#protocolVersion=3".to_owned(),
            source_mapped_package_patterns: None,
        }]);
        assert!(xml.contains(r#"value="https://my-registry.example.org/" protocolVersion="3""#));
        assert!(!xml.contains("#protocolVersion"));
    }

    // Ported: "skips duplicate registry URLs" — manager/nuget/config-formatter.spec.ts line 265
    #[test]
    fn nuget_config_xml_skips_duplicates() {
        crate::util::host_rules::clear();
        let xml = create_nuget_config_xml(&[
            NuGetRegistry {
                name: Some("myRegistry".to_owned()),
                url: "https://my-registry.example.org".to_owned(),
                source_mapped_package_patterns: None,
            },
            NuGetRegistry {
                name: Some("duplicateRegistry".to_owned()),
                url: "https://my-registry.example.org".to_owned(),
                source_mapped_package_patterns: None,
            },
            NuGetRegistry {
                name: None,
                url: "https://my-unnamed-registry.example.org/index.json".to_owned(),
                source_mapped_package_patterns: None,
            },
        ]);
        assert!(xml.contains(r#"key="myRegistry""#));
        assert!(!xml.contains(r#"key="duplicateRegistry""#));
        assert!(xml.contains(r#"key="Package source 1""#));
    }

    // Ported: "includes packageSourceMapping when defined" — manager/nuget/config-formatter.spec.ts line 202
    #[test]
    fn nuget_config_xml_package_source_mapping() {
        crate::util::host_rules::clear();
        let xml = create_nuget_config_xml(&[
            NuGetRegistry {
                name: Some("myRegistry".to_owned()),
                url: "https://my-registry.example.org".to_owned(),
                source_mapped_package_patterns: Some(vec!["*".to_owned()]),
            },
            NuGetRegistry {
                name: Some("myRegistry2".to_owned()),
                url: "https://my-registry2.example.org/index.json".to_owned(),
                source_mapped_package_patterns: Some(vec![
                    "LimitedPackages.*".to_owned(),
                    "MySpecialPackage".to_owned(),
                ]),
            },
        ]);
        assert!(xml.contains("packageSourceMapping"));
        assert!(xml.contains(r#"<packageSource key="myRegistry">"#));
        assert!(xml.contains(r#"<package pattern="*" />"#));
        assert!(xml.contains(r#"<packageSource key="myRegistry2">"#));
        assert!(xml.contains(r#"<package pattern="LimitedPackages.*" />"#));
        assert!(xml.contains(r#"<package pattern="MySpecialPackage" />"#));
    }

    // Ported: "excludes packageSourceMapping when undefined" — manager/nuget/config-formatter.spec.ts line 245
    #[test]
    fn nuget_config_xml_no_package_source_mapping() {
        crate::util::host_rules::clear();
        let xml = create_nuget_config_xml(&[NuGetRegistry {
            name: Some("myRegistry".to_owned()),
            url: "https://my-registry.example.org".to_owned(),
            source_mapped_package_patterns: None,
        }]);
        assert!(!xml.contains("packageSourceMapping"));
    }

    // Ported: "returns xml with authenticated registries" — manager/nuget/config-formatter.spec.ts line 58
    #[test]
    fn nuget_config_xml_with_credentials() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(crate::util::host_rules::HostRule {
            host_type: Some("nuget".to_owned()),
            match_host: Some("my-registry.example.org".to_owned()),
            username: Some("some-username".to_owned()),
            password: Some("some-password".to_owned()),
            ..Default::default()
        })
        .unwrap();
        let xml = create_nuget_config_xml(&[NuGetRegistry {
            name: Some("myRegistry".to_owned()),
            url: "https://my-registry.example.org".to_owned(),
            source_mapped_package_patterns: None,
        }]);
        assert!(xml.contains("packageSourceCredentials"));
        assert!(xml.contains(r#"<add key="Username" value="some-username" />"#));
        assert!(xml.contains(r#"<add key="ClearTextPassword" value="some-password" />"#));
        assert!(xml.contains(r#"<add key="ValidAuthenticationTypes" value="basic" />"#));
    }

    // Ported: "escapes registry credential names containing special characters" — manager/nuget/config-formatter.spec.ts line 138
    #[test]
    fn nuget_config_xml_escapes_special_chars_in_names() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(crate::util::host_rules::HostRule {
            host_type: Some("nuget".to_owned()),
            match_host: Some("my-registry.example.org".to_owned()),
            username: Some("some-username".to_owned()),
            password: Some("some-password".to_owned()),
            ..Default::default()
        })
        .unwrap();
        let xml = create_nuget_config_xml(&[NuGetRegistry {
            name: Some("my very? weird!-regi$try_name".to_owned()),
            url: "https://my-registry.example.org".to_owned(),
            source_mapped_package_patterns: None,
        }]);
        assert!(xml.contains("my_x0020_very_x003f__x0020_weird_x0021_-regi_x0024_try_name"));
        assert!(xml.contains(r#"<add key="Username" value="some-username" />"#));
    }

    #[test]
    fn parse_nuget_registry_url_default() {
        let parsed = parse_nuget_registry_url("https://nuget.org");
        assert_eq!(parsed.feed_url, "https://nuget.org/");
        assert_eq!(parsed.protocol_version, 2);
    }

    #[test]
    fn parse_nuget_registry_url_v3() {
        let parsed = parse_nuget_registry_url("https://nuget.org/index.json");
        assert_eq!(parsed.protocol_version, 3);
    }

    #[test]
    fn parse_nuget_registry_url_fragment() {
        let parsed = parse_nuget_registry_url("https://nuget.org#protocolVersion=3");
        assert_eq!(parsed.feed_url, "https://nuget.org/");
        assert_eq!(parsed.protocol_version, 3);
    }

    // ── nuget/util.spec.ts ───────────────────────────────────────────────────

    // Ported: "finds the version in a later property group" — nuget/util.spec.ts line 17
    #[test]
    fn find_version_in_later_property_group() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup><PropertyGroup><Version>0.0.2</Version></PropertyGroup></Project>"#;
        let version = find_version_in_csproj(content);
        assert_eq!(version, Some("0.0.2".to_owned()));
    }

    // Ported: "picks version over versionprefix" — nuget/util.spec.ts line 28
    #[test]
    fn find_version_prefers_version_over_versionprefix() {
        let content = r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><VersionPrefix>0.0.5</VersionPrefix></PropertyGroup><PropertyGroup><Version>0.0.2</Version></PropertyGroup></Project>"#;
        let version = find_version_in_csproj(content);
        assert_eq!(version, Some("0.0.2".to_owned()));
    }

    // Ported: "reads nuget config file" — nuget/util.spec.ts line 41
    #[test]
    fn get_configured_registries_reads_nuget_config() {
        let config = r#"<configuration><packageSources><clear/><add key="nuget.org" value="https://api.nuget.org/v3/index.json"/><add key="contoso.com" value="https://contoso.com/packages/"/></packageSources></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert_eq!(registries.len(), 2);
        assert_eq!(registries[0].url, "https://api.nuget.org/v3/index.json");
        assert_eq!(registries[1].url, "https://contoso.com/packages/");
    }

    // Ported: "deduplicates registries" — nuget/util.spec.ts line 78
    #[test]
    fn get_configured_registries_deduplicates() {
        let config = r#"<configuration><packageSources><add key="nuget.org" value="https://api.nuget.org/v3/index.json" protocolVersion="3"/></packageSources></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert_eq!(registries.len(), 1);
        assert_eq!(registries[0].url, "https://api.nuget.org/v3/index.json#protocolVersion=3");
    }

    // Ported: "reads nuget config file with default registry" — nuget/util.spec.ts line 99
    #[test]
    fn get_configured_registries_with_default_registry() {
        let config = r#"<configuration><packageSources><add key="contoso.com" value="https://contoso.com/packages/"/></packageSources><disabledPackageSources/></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert_eq!(registries.len(), 2);
        assert_eq!(registries[0].name.as_deref(), Some("nuget.org"));
        assert_eq!(registries[1].name.as_deref(), Some("contoso.com"));
    }

    // Ported: "reads nuget config file with default registry disabled and added sources" — nuget/util.spec.ts line 134
    #[test]
    fn get_configured_registries_default_disabled_with_added_sources() {
        let config = r#"<configuration><packageSources><clear/><add key="contoso" value="https://contoso.com/packages/"/></packageSources><disabledPackageSources><add key="nuget.org"/></disabledPackageSources></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert_eq!(registries.len(), 1);
        assert_eq!(registries[0].url, "https://contoso.com/packages/");
    }

    // Ported: "reads nuget config file with default registry disabled given default registry added" — nuget/util.spec.ts line 157
    #[test]
    fn get_configured_registries_default_disabled_given_default_added() {
        let config = r#"<configuration><packageSources><clear/><add key="nuget.org" value="https://api.nuget.org/v3/index.json"/></packageSources><disabledPackageSources><add key="nuget.org"/></disabledPackageSources></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert_eq!(registries.len(), 1);
        assert_eq!(registries[0].url, "https://api.nuget.org/v3/index.json");
    }

    // Ported: "reads nuget config file with unknown disabled source" — nuget/util.spec.ts line 181
    #[test]
    fn get_configured_registries_unknown_disabled_source() {
        let config = r#"<configuration><packageSources><add key="nuget.org" value="https://api.nuget.org/v3/index.json"/></packageSources><disabledPackageSources><add key="unknown"/></disabledPackageSources></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert_eq!(registries.len(), 2); // default + nuget.org
    }

    // Ported: "reads nuget config file with disabled source with value false" — nuget/util.spec.ts line 208
    #[test]
    fn get_configured_registries_disabled_source_with_value_false() {
        let config = r#"<configuration><packageSources><add key="nuget.org" value="https://api.nuget.org/v3/index.json"/></packageSources><disabledPackageSources><add key="nuget.org" value="false"/></disabledPackageSources></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert_eq!(registries.len(), 2);
    }

    // Ported: "reads nuget config file without packageSources and ignores disabledPackageSources" — nuget/util.spec.ts line 237
    #[test]
    fn get_configured_registries_no_package_sources() {
        let config = r#"<configuration><disabledPackageSources><add key="nuget.org"/></disabledPackageSources></configuration>"#;
        let registries = parse_nuget_config_registries_full(config);
        assert!(registries.is_empty());
    }

    // Ported: "applies registry to package name via source mapping" — nuget/util.spec.ts line 254
    #[test]
    fn apply_registries_with_source_mapping() {
        let mut dep = NuGetPackageDependency {
            dep_name: "Newtonsoft.Json".to_owned(),
            ..Default::default()
        };
        let registries = vec![
            NuGetRegistry {
                name: Some("nuget.org".to_owned()),
                url: "https://api.nuget.org/v3/index.json".to_owned(),
                source_mapped_package_patterns: Some(vec!["*".to_owned()]),
            },
        ];
        apply_registries_to_dep(&mut dep, &registries);
        assert_eq!(dep.registry_urls, Some(vec!["https://api.nuget.org/v3/index.json".to_owned()]));
    }

    // Ported: "applies registry to package name case insensitive" — nuget/util.spec.ts line 323
    #[test]
    fn apply_registries_case_insensitive() {
        let mut dep = NuGetPackageDependency {
            dep_name: "newtonsoft.json".to_owned(),
            ..Default::default()
        };
        let registries = vec![
            NuGetRegistry {
                name: Some("nuget.org".to_owned()),
                url: "https://api.nuget.org/v3/index.json".to_owned(),
                source_mapped_package_patterns: Some(vec!["Newtonsoft*".to_owned()]),
            },
        ];
        apply_registries_to_dep(&mut dep, &registries);
        assert_eq!(dep.registry_urls, Some(vec!["https://api.nuget.org/v3/index.json".to_owned()]));
    }

    // Ported: "applies all registries to package name" — nuget/util.spec.ts line 343
    #[test]
    fn apply_registries_all_matching() {
        let mut dep = NuGetPackageDependency {
            dep_name: "Newtonsoft.Json".to_owned(),
            ..Default::default()
        };
        let registries = vec![
            NuGetRegistry {
                name: Some("nuget.org".to_owned()),
                url: "https://api.nuget.org/v3/index.json".to_owned(),
                source_mapped_package_patterns: None,
            },
            NuGetRegistry {
                name: Some("contoso".to_owned()),
                url: "https://contoso.com/packages/".to_owned(),
                source_mapped_package_patterns: None,
            },
        ];
        apply_registries_to_dep(&mut dep, &registries);
        assert_eq!(dep.registry_urls, Some(vec![
            "https://api.nuget.org/v3/index.json".to_owned(),
            "https://contoso.com/packages/".to_owned(),
        ]));
    }

    // Ported: "applies nothing" — nuget/util.spec.ts line 371
    #[test]
    fn apply_registries_no_match() {
        let mut dep = NuGetPackageDependency {
            dep_name: "Newtonsoft.Json".to_owned(),
            ..Default::default()
        };
        apply_registries_to_dep(&mut dep, &[]);
        assert!(dep.registry_urls.is_none());
    }

    // Ported: "not found" — nuget/util.spec.ts line 386
    #[test]
    fn find_global_json_not_found() {
        assert!(extract_global_json("{}").is_none());
    }

    // Ported: "no content" — nuget/util.spec.ts line 392
    #[test]
    fn find_global_json_no_content() {
        assert!(extract_global_json("").is_none());
    }

    // Ported: "fails to parse" — nuget/util.spec.ts line 398
    #[test]
    fn find_global_json_fails_to_parse() {
        assert!(extract_global_json("{{").is_none());
    }

    // Ported: "parses" — nuget/util.spec.ts line 405
    #[test]
    fn find_global_json_parses() {
        let content = r#"{"sdk": {"version": "5.0.302", "rollForward": "latestMajor"}, "msbuild-sdks": {"My.Custom.Sdk": "5.0.0"}}"#;
        let result = extract_global_json(content).unwrap();
        assert_eq!(result.dotnet_sdk_constraint, Some("5.0.302".to_owned()));
        assert_eq!(result.deps.len(), 2);
    }

    // ── package-tree tests ──────────────────────────────────────────────────

    // Ported: "returns self for single project" — manager/nuget/package-tree.spec.ts line 32
    #[test]
    fn package_tree_returns_self_for_single_project() {
        let mut files = std::collections::HashMap::new();
        files.insert("single.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let result = get_dependent_package_files_pure("single.csproj", &["single.csproj".to_owned()], &files, false, false).unwrap();
        assert_eq!(result, vec![ProjectFile { name: "single.csproj".to_owned(), is_leaf: true }]);
    }

    // Ported: "returns self for two projects with no references" — manager/nuget/package-tree.spec.ts line 45
    #[test]
    fn package_tree_returns_self_for_two_projects_no_references() {
        let mut files = std::collections::HashMap::new();
        files.insert("one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        files.insert("two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["one.csproj".to_owned(), "two.csproj".to_owned()];
        let result = get_dependent_package_files_pure("one.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![ProjectFile { name: "one.csproj".to_owned(), is_leaf: true }]);
        let result = get_dependent_package_files_pure("two.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![ProjectFile { name: "two.csproj".to_owned(), is_leaf: true }]);
    }

    // Ported: "returns projects for two projects with one reference" — manager/nuget/package-tree.spec.ts line 60
    #[test]
    fn package_tree_returns_projects_for_two_with_one_reference() {
        let mut files = std::collections::HashMap::new();
        files.insert("one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../two/two.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("two/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["one/one.csproj".to_owned(), "two/two.csproj".to_owned()];
        let result = get_dependent_package_files_pure("one/one.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "one/one.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: true },
        ]);
    }

    // Ported: "returns project for two projects with one reference and central versions" — manager/nuget/package-tree.spec.ts line 77
    #[test]
    fn package_tree_returns_projects_with_central_versions() {
        let mut files = std::collections::HashMap::new();
        files.insert("one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../two/two.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("two/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["one/one.csproj".to_owned(), "two/two.csproj".to_owned()];
        let result = get_dependent_package_files_pure("Directory.Packages.props", &file_list, &files, true, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "one/one.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: true },
        ]);
    }

    // Ported: "returns projects for two projects with one reference and Directory.Build.props" — manager/nuget/package-tree.spec.ts line 99
    #[test]
    fn package_tree_returns_projects_with_directory_build_props() {
        let mut files = std::collections::HashMap::new();
        files.insert("one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../two/two.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("two/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["one/one.csproj".to_owned(), "two/two.csproj".to_owned()];
        let result = get_dependent_package_files_pure("Directory.Build.props", &file_list, &files, true, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "one/one.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: true },
        ]);
    }

    // Ported: "returns only projects under nested Directory.Build.props directory" — manager/nuget/package-tree.spec.ts line 121
    #[test]
    fn package_tree_returns_only_projects_under_nested_props() {
        let mut files = std::collections::HashMap::new();
        files.insert("src/one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        files.insert("other/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["src/one/one.csproj".to_owned(), "other/two.csproj".to_owned()];
        let result = get_dependent_package_files_pure("src/Directory.Build.props", &file_list, &files, true, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "src/one/one.csproj".to_owned(), is_leaf: true },
        ]);
    }

    // Ported: "returns project for two projects with one reference and global.json" — manager/nuget/package-tree.spec.ts line 143
    #[test]
    fn package_tree_returns_projects_with_global_json() {
        let mut files = std::collections::HashMap::new();
        files.insert("one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../two/two.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("two/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["one/one.csproj".to_owned(), "two/two.csproj".to_owned()];
        let result = get_dependent_package_files_pure("global.json", &file_list, &files, false, true).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "one/one.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: true },
        ]);
    }

    // Ported: "returns projects for three projects with two linear references" — manager/nuget/package-tree.spec.ts line 163
    #[test]
    fn package_tree_returns_projects_three_linear() {
        let mut files = std::collections::HashMap::new();
        files.insert("one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../two/two.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("two/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../three/three.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("three/three.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["one/one.csproj".to_owned(), "two/two.csproj".to_owned(), "three/three.csproj".to_owned()];

        let result = get_dependent_package_files_pure("one/one.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "one/one.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "three/three.csproj".to_owned(), is_leaf: true },
        ]);

        let result = get_dependent_package_files_pure("two/two.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "three/three.csproj".to_owned(), is_leaf: true },
        ]);

        let result = get_dependent_package_files_pure("three/three.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "three/three.csproj".to_owned(), is_leaf: true },
        ]);
    }

    // Ported: "returns projects for three projects with two tree-like references" — manager/nuget/package-tree.spec.ts line 197
    #[test]
    fn package_tree_returns_projects_three_treelike() {
        let mut files = std::collections::HashMap::new();
        files.insert("one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../two/two.csproj" /><ProjectReference Include="../three/three.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("two/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        files.insert("three/three.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net6.0</TargetFramework></PropertyGroup></Project>"#.to_owned());
        let file_list = vec!["one/one.csproj".to_owned(), "two/two.csproj".to_owned(), "three/three.csproj".to_owned()];

        let result = get_dependent_package_files_pure("one/one.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "one/one.csproj".to_owned(), is_leaf: false },
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: true },
            ProjectFile { name: "three/three.csproj".to_owned(), is_leaf: true },
        ]);

        let result = get_dependent_package_files_pure("two/two.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "two/two.csproj".to_owned(), is_leaf: true },
        ]);

        let result = get_dependent_package_files_pure("three/three.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![
            ProjectFile { name: "three/three.csproj".to_owned(), is_leaf: true },
        ]);
    }

    // Ported: "throws error on circular reference" — manager/nuget/package-tree.spec.ts line 229
    #[test]
    fn package_tree_throws_on_circular_reference() {
        let mut files = std::collections::HashMap::new();
        files.insert("one/one.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../two/two.csproj" /></ItemGroup></Project>"#.to_owned());
        files.insert("two/two.csproj".to_owned(), r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../one/one.csproj" /></ItemGroup></Project>"#.to_owned());
        let file_list = vec!["one/one.csproj".to_owned(), "two/two.csproj".to_owned()];
        let result = get_dependent_package_files_pure("one/one.csproj", &file_list, &files, false, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Circular reference"));
    }

    // Ported: "skips on invalid xml file" — manager/nuget/package-tree.spec.ts line 245
    #[test]
    fn package_tree_skips_invalid_xml() {
        let mut files = std::collections::HashMap::new();
        files.insert("foo/bar.csproj".to_owned(), "<invalid".to_owned());
        let file_list = vec!["foo/bar.csproj".to_owned()];
        let result = get_dependent_package_files_pure("foo/bar.csproj", &file_list, &files, false, false).unwrap();
        assert_eq!(result, vec![ProjectFile { name: "foo/bar.csproj".to_owned(), is_leaf: true }]);
    }
}

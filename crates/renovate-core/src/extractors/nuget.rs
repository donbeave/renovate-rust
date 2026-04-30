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

use std::io::BufReader;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
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
}

impl NuGetDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            NuGetDepType::Package => "PackageReference",
            NuGetDepType::PackageVersion => "PackageVersion",
            NuGetDepType::CliTool => "DotNetCliToolReference",
            NuGetDepType::Global => "GlobalPackageReference",
            NuGetDepType::MsbuildSdk => "msbuild-sdk",
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
    /// Which element type this came from.
    pub dep_type: NuGetDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<NuGetSkipReason>,
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
                if let Some(dep_type) = dep_type_for(&elem) {
                    current = attrs_to_pending(e, dep_type);
                } else if current.is_some() && (elem == "Version" || elem == "VersionOverride") {
                    version_child_tag = Some(elem);
                }
            }

            Event::End(e) => {
                let elem = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if version_child_tag.as_deref() == Some(elem.as_str()) {
                    version_child_tag = None;
                } else if dep_type_for(&elem).is_some()
                    && let Some(pending) = current.take()
                    && !pending.package_id.is_empty()
                {
                    deps.push(build_dep(pending));
                }
            }

            Event::Text(e) => {
                if let Some(ref tag) = version_child_tag {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if !text.is_empty()
                        && let Some(ref mut pending) = current
                        && (tag == "VersionOverride" || pending.version.is_empty())
                    {
                        pending.version = text;
                    }
                }
            }

            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(deps)
}

// ── Helpers ───────────────────────────────────────────────────────────────

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
            dep_type: dep.dep_type,
            skip_reason: Some(NuGetSkipReason::NoVersion),
        };
    }

    if dep.version.contains("$(") {
        return NuGetExtractedDep {
            package_id: dep.package_id,
            current_value: dep.version,
            dep_type: dep.dep_type,
            skip_reason: Some(NuGetSkipReason::PropertyRef),
        };
    }

    let (current_value, skip_reason) = normalize_version(&dep.version);
    NuGetExtractedDep {
        package_id: dep.package_id,
        current_value,
        dep_type: dep.dep_type,
        skip_reason,
    }
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
}

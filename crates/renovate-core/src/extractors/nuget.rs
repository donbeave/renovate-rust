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
                if let Some(dep_type) = dep_type_for(&elem)
                    && let Some(pending) = attrs_to_pending(e, dep_type)
                    && !pending.package_id.is_empty()
                {
                    deps.push(build_dep(pending));
                }
            }

            // Opening element: may have child elements.
            Event::Start(ref e) => {
                let elem = elem_name(e);
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
        match key.as_str() {
            "Include" | "Update" => package_id = val,
            "Version" => version = val,
            "VersionOverride" => version_override = val,
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

    #[test]
    fn empty_project_returns_empty() {
        let deps = extract_ok(r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup/></Project>"#);
        assert!(deps.is_empty());
    }

    #[test]
    fn normalize_plain_version() {
        assert_eq!(normalize_version("1.2.3"), ("1.2.3".to_owned(), None));
    }

    #[test]
    fn normalize_exact_range() {
        assert_eq!(normalize_version("[1.2.3]"), ("1.2.3".to_owned(), None));
    }

    #[test]
    fn normalize_minimum_ranges() {
        assert_eq!(normalize_version("[1.2.3,]"), ("1.2.3".to_owned(), None));
        assert_eq!(normalize_version("[1.2.3,)"), ("1.2.3".to_owned(), None));
    }
}

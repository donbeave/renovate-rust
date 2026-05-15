//! Gleam `gleam.toml` dependency extractor.
//!
//! Parses `dependencies` and `dev-dependencies` sections of `gleam.toml`
//! files and maps each entry to the Hex.pm datasource.
//!
//! Renovate reference:
//! - `lib/modules/manager/gleam/extract.ts`
//! - Pattern: `/(^|/)gleam\.toml$/`
//!
//! ## Supported form
//!
//! ```toml
//! [dependencies]
//! gleam_stdlib = "~> 0.34"
//! lustre = ">= 4.0.0, < 5.0.0"
//!
//! [dev-dependencies]
//! gleeunit = "~> 1.0"
//! ```

use std::collections::HashMap;

/// A single extracted Gleam dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GleamDep {
    pub name: String,
    pub version: String,
    pub dev: bool,
}

/// Parse `gleam.toml` and extract all Hex.pm dependencies.
pub fn extract(content: &str) -> Vec<GleamDep> {
    let mut out = Vec::new();
    let mut in_deps = false;
    let mut in_dev_deps = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Section headers.
        if trimmed.starts_with('[') {
            in_deps = trimmed == "[dependencies]";
            in_dev_deps = trimmed == "[dev-dependencies]";
            continue;
        }

        if !in_deps && !in_dev_deps {
            continue;
        }

        // Parse `name = "version"` entries.
        if let Some((name_raw, val_raw)) = trimmed.split_once('=') {
            let name = name_raw.trim();
            let version = val_raw.trim().trim_matches('"').trim_matches('\'').trim();
            if !name.is_empty() && !version.is_empty() {
                out.push(GleamDep {
                    name: name.to_owned(),
                    version: version.to_owned(),
                    dev: in_dev_deps,
                });
            }
        }
    }

    out
}

/// A package entry from Gleam's `manifest.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GleamLockPackage {
    pub name: String,
    pub version: String,
    pub requirements: Vec<String>,
}

/// Parsed Gleam `manifest.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GleamLock {
    pub packages: Vec<GleamLockPackage>,
}

/// Parse a Gleam `manifest.toml` string.
///
/// Mirrors `lib/modules/manager/gleam/locked-version.ts` `parseLockFile()`.
pub fn parse_gleam_lock_file(content: &str) -> Option<GleamLock> {
    let table = content.parse::<toml::Table>().ok()?;
    let packages_val = table.get("packages");
    let packages = match packages_val {
        None => vec![],
        Some(v) => {
            let arr = v.as_array()?;
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                let t = item.as_table()?;
                let name = t.get("name")?.as_str()?.to_owned();
                let version = t.get("version")?.as_str()?.to_owned();
                let requirements = t
                    .get("requirements")
                    .and_then(|r| r.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(str::to_owned)
                            .collect()
                    })
                    .unwrap_or_default();
                out.push(GleamLockPackage {
                    name,
                    version,
                    requirements,
                });
            }
            out
        }
    };
    Some(GleamLock { packages })
}

/// Extract a map of package name → [versions] from a Gleam `manifest.toml` string.
///
/// Mirrors `lib/modules/manager/gleam/locked-version.ts` `extractLockFileVersions()`.
pub fn extract_gleam_lock_file_versions(content: &str) -> Option<HashMap<String, Vec<String>>> {
    let lock = parse_gleam_lock_file(content)?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for pkg in lock.packages {
        map.entry(pkg.name).or_default().push(pkg.version);
    }
    Some(map)
}

/// Determine the effective Gleam range strategy.
///
/// Mirrors `lib/modules/manager/gleam/range.ts` `getRangeStrategy()`.
pub fn get_range_strategy<'a>(range_strategy: &'a str, current_value: Option<&str>) -> &'a str {
    let is_complex = current_value
        .is_some_and(|v| v.contains(" and ") || v.contains(" or ") || v.contains("||"));
    if range_strategy == "bump" && is_complex {
        return "widen";
    }
    if range_strategy != "auto" {
        return range_strategy;
    }
    "widen"
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should extract dev and prod dependencies" — gleam/extract.spec.ts line 8
    #[test]
    fn extracts_dependencies() {
        let content = r#"
[dependencies]
gleam_stdlib = "~> 0.34"
lustre = ">= 4.0.0, < 5.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "gleam_stdlib" && !d.dev));
        assert!(deps.iter().any(|d| d.name == "lustre" && !d.dev));
    }

    // Ported: "should extract dev only dependencies" — gleam/extract.spec.ts line 41
    #[test]
    fn extracts_dev_dependencies() {
        let content = r#"
[dev-dependencies]
gleeunit = "~> 1.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].dev);
        assert_eq!(deps[0].name, "gleeunit");
    }

    // Ported: "should extract dev and prod dependencies" — gleam/extract.spec.ts line 8
    #[test]
    fn both_sections() {
        let content = r#"
[dependencies]
gleam_stdlib = "~> 0.34"

[dev-dependencies]
gleeunit = "~> 1.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }

    // Ported: "should return null when gleam.toml is invalid" — gleam/extract.spec.ts line 82
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "should return null when no dependencies are found" — gleam/extract.spec.ts line 65
    #[test]
    fn no_deps_section_returns_empty() {
        let content = r#"name = "test"\nversion = "1.0.0"\n\n[unknown]\ngleam_http = "~> 3.6.0""#;
        assert!(extract(content).is_empty());
    }

    // Ported: "should return null when gleam.toml is invalid" — gleam/extract.spec.ts line 82
    #[test]
    fn invalid_toml_returns_empty() {
        assert!(extract("foo").is_empty());
    }

    // Ported: "returns same if not auto" — modules/manager/gleam/range.spec.ts line 4
    #[test]
    fn gleam_range_returns_same_if_not_auto() {
        assert_eq!(get_range_strategy("pin", None), "pin");
    }

    // Ported: "widens complex bump" — modules/manager/gleam/range.spec.ts line 9
    #[test]
    fn gleam_range_widens_complex_bump() {
        let result = get_range_strategy("bump", Some(">= 1.6.0 and < 2.0.0"));
        assert_eq!(result, "widen");
    }

    // Ported: "defaults to widen" — modules/manager/gleam/range.spec.ts line 18
    #[test]
    fn gleam_range_defaults_to_widen() {
        let result = get_range_strategy("auto", None);
        assert_eq!(result, "widen");
    }

    const LOCK_FILE: &str = r#"packages = [
  { name = "foo", version = "1.0.4", build_tools = ["gleam"], requirements = ["bar"], otp_app = "foo", source = "hex", outer_checksum = "ABC" },
  { name = "bar", version = "2.1.0", build_tools = ["rebar3"], requirements = [], otp_app = "bar", source = "hex", outer_checksum = "DEF" },
]

[requirements]
foo = { version = ">= 1.0.0 and < 2.0.0" }
"#;

    // Ported: "returns null for invalid lock file" — modules/manager/gleam/locked-version.spec.ts line 26
    #[test]
    fn gleam_lock_returns_none_for_invalid() {
        assert!(parse_gleam_lock_file("foo").is_none());
    }

    // Ported: "returns empty map for lock file without packages" — modules/manager/gleam/locked-version.spec.ts line 31
    #[test]
    fn gleam_lock_returns_empty_map_for_no_packages() {
        let result = extract_gleam_lock_file_versions("[requirements]").unwrap();
        assert!(result.is_empty());
    }

    // Ported: "returns a map of package versions" — modules/manager/gleam/locked-version.spec.ts line 36
    #[test]
    fn gleam_lock_returns_map_of_package_versions() {
        let result = extract_gleam_lock_file_versions(LOCK_FILE).unwrap();
        assert_eq!(result.get("foo"), Some(&vec!["1.0.4".to_string()]));
        assert_eq!(result.get("bar"), Some(&vec!["2.1.0".to_string()]));
    }

    // Ported: "parses lockfile string into an object" — modules/manager/gleam/locked-version.spec.ts line 47
    #[test]
    fn gleam_lock_parses_into_object() {
        let result = parse_gleam_lock_file(LOCK_FILE).unwrap();
        assert_eq!(result.packages.len(), 2);
        assert_eq!(result.packages[0].name, "foo");
        assert_eq!(result.packages[0].version, "1.0.4");
        assert_eq!(result.packages[0].requirements, vec!["bar"]);
        assert_eq!(result.packages[1].name, "bar");
        assert_eq!(result.packages[1].version, "2.1.0");
        assert!(result.packages[1].requirements.is_empty());
    }

    // Ported: "can deal with invalid lockfiles" — modules/manager/gleam/locked-version.spec.ts line 65
    #[test]
    fn gleam_lock_handles_invalid_lockfile() {
        assert!(parse_gleam_lock_file("foo").is_none());
    }
}

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

use semver::Version;

/// A single extracted Gleam dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GleamDep {
    pub name: String,
    pub version: String,
    pub dev: bool,
    /// Locked version from `manifest.toml`, if available and satisfies the constraint.
    pub locked_version: Option<String>,
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
                    locked_version: None,
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

/// Wrapper accepting an optional content string; returns `None` when the file
/// is absent (mirrors the missing-file branch of `extractLockFileVersions`).
pub fn extract_gleam_lock_file_versions_opt(
    content: Option<&str>,
) -> Option<HashMap<String, Vec<String>>> {
    extract_gleam_lock_file_versions(content?)
}

/// Check whether `version` satisfies a Hex-style range string.
///
/// Handles constraints like `>= 1.0.0 and < 2.0.0` or `~> 1.2`.
/// Returns `false` for unparseable versions or constraints.
pub fn hex_version_satisfies(version_str: &str, constraint: &str) -> bool {
    let Ok(ver) = Version::parse(version_str) else {
        return false;
    };
    // Split on ` and ` / ` or ` / `&&` / `||`
    let parts: Vec<&str> = constraint
        .split(" and ")
        .flat_map(|s| s.split(" or "))
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    // Collect comparators; all must be satisfied (treat as AND for each segment)
    for part in &parts {
        if !single_constraint_matches(&ver, part) {
            return false;
        }
    }
    true
}

fn single_constraint_matches(ver: &Version, constraint: &str) -> bool {
    let trimmed = constraint.trim();
    // Pessimistic range: `~> X.Y` means `>= X.Y.0 and < X+1.0.0`
    //                    `~> X.Y.Z` means `>= X.Y.Z and < X.Y+1.0`
    if let Some(rest) = trimmed.strip_prefix("~>").map(str::trim) {
        let parts: Vec<u64> = rest.split('.').filter_map(|s| s.parse().ok()).collect();
        return match parts.as_slice() {
            [major, minor] => {
                *ver >= Version::new(*major, *minor, 0)
                    && *ver < Version::new(*major + 1, 0, 0)
            }
            [major, minor, patch] => {
                *ver >= Version::new(*major, *minor, *patch)
                    && *ver < Version::new(*major, *minor + 1, 0)
            }
            _ => false,
        };
    }
    // Standard operators: >=, <=, >, <, ==, !=
    let (op, ver_str) = if let Some(r) = trimmed.strip_prefix(">=").map(str::trim) {
        (">=", r)
    } else if let Some(r) = trimmed.strip_prefix("<=").map(str::trim) {
        ("<=", r)
    } else if let Some(r) = trimmed.strip_prefix("!=").map(str::trim) {
        ("!=", r)
    } else if let Some(r) = trimmed.strip_prefix('>').map(str::trim) {
        (">", r)
    } else if let Some(r) = trimmed.strip_prefix('<').map(str::trim) {
        ("<", r)
    } else if let Some(r) = trimmed.strip_prefix("==").map(str::trim) {
        ("==", r)
    } else {
        ("==", trimmed)
    };
    let Ok(bound) = Version::parse(ver_str) else {
        return false;
    };
    match op {
        ">=" => *ver >= bound,
        "<=" => *ver <= bound,
        ">" => *ver > bound,
        "<" => *ver < bound,
        "!=" => *ver != bound,
        _ => *ver == bound,
    }
}

/// Extract deps from `gleam.toml` and populate `locked_version` from an
/// optional `manifest.toml` content.
///
/// Mirrors the async `extractPackageFile` in TypeScript that reads the sibling
/// lock file and calls `getLockedVersion`.
pub fn extract_with_lock(toml_content: &str, lock_content: Option<&str>) -> Vec<GleamDep> {
    let mut deps = extract(toml_content);
    let versions = extract_gleam_lock_file_versions_opt(lock_content);
    if let Some(ref vmap) = versions {
        for dep in &mut deps {
            if let Some(candidates) = vmap.get(&dep.name) {
                // Find the first candidate version that satisfies the constraint.
                let locked = candidates
                    .iter()
                    .find(|v| hex_version_satisfies(v, &dep.version));
                dep.locked_version = locked.cloned();
            }
        }
    }
    deps
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

    const GLEAM_TOML: &str = "name = \"test_gleam_toml\"\nversion = \"1.0.0\"\n\n[dependencies]\nfoo = \">= 1.0.0 and < 2.0.0\"\n";

    const LOCK_FILE_SATISFYING: &str = concat!(
        "packages = [\n",
        "  { name = \"foo\", version = \"1.0.4\", build_tools = [\"gleam\"], requirements = [\"bar\"], otp_app = \"foo\", source = \"hex\", outer_checksum = \"AAA\" },\n",
        "  { name = \"bar\", version = \"2.1.0\", build_tools = [\"rebar3\"], requirements = [], otp_app = \"bar\", source = \"hex\", outer_checksum = \"BBB\" },\n",
        "]\n\n[requirements]\nfoo = { version = \">= 1.0.0 and < 2.0.0\" }\n"
    );

    const LOCK_FILE_OUT_OF_RANGE: &str = concat!(
        "packages = [\n",
        "  { name = \"foo\", version = \"2.0.1\", build_tools = [\"gleam\"], requirements = [], otp_app = \"foo\", source = \"hex\", outer_checksum = \"AAA\" },\n",
        "]\n\n[requirements]\nfoo = { version = \">= 1.0.0 and < 2.0.0\" }\n"
    );

    const LOCK_FILE_INVALID_VERSION: &str = concat!(
        "packages = [\n",
        "  { name = \"foo\", version = \"fooey\", build_tools = [\"gleam\"], requirements = [], otp_app = \"foo\", source = \"hex\", outer_checksum = \"AAA\" },\n",
        "]\n\n[requirements]\nfoo = { version = \">= 1.0.0 and < 2.0.0\" }\n"
    );

    // Ported: "should return locked versions" — gleam/extract.spec.ts line 91
    #[test]
    fn gleam_extract_returns_locked_versions() {
        let deps = extract_with_lock(GLEAM_TOML, Some(LOCK_FILE_SATISFYING));
        assert!(!deps.is_empty(), "expected deps");
        assert!(
            deps.iter().all(|d| d.locked_version.is_some()),
            "all deps should have lockedVersion"
        );
        let foo = deps.iter().find(|d| d.name == "foo").unwrap();
        assert_eq!(foo.locked_version.as_deref(), Some("1.0.4"));
    }

    // Ported: "should fail to extract locked version" — gleam/extract.spec.ts line 119
    #[test]
    fn gleam_extract_no_lock_file_no_locked_version() {
        let deps = extract_with_lock(GLEAM_TOML, None);
        assert!(!deps.is_empty(), "expected deps");
        assert!(
            deps.iter().all(|d| d.locked_version.is_none()),
            "no deps should have lockedVersion"
        );
    }

    // Ported: "should fail to find locked version in range" — gleam/extract.spec.ts line 138
    #[test]
    fn gleam_extract_locked_version_out_of_range() {
        let deps = extract_with_lock(GLEAM_TOML, Some(LOCK_FILE_OUT_OF_RANGE));
        assert!(!deps.is_empty(), "expected deps");
        let foo = deps.iter().find(|d| d.name == "foo").unwrap();
        assert!(foo.locked_version.is_none(), "out-of-range version not set");
    }

    // Ported: "should handle invalid versions in lock file" — gleam/extract.spec.ts line 166
    #[test]
    fn gleam_extract_invalid_lock_version() {
        let deps = extract_with_lock(GLEAM_TOML, Some(LOCK_FILE_INVALID_VERSION));
        assert!(!deps.is_empty(), "expected deps");
        let foo = deps.iter().find(|d| d.name == "foo").unwrap();
        assert!(foo.locked_version.is_none(), "invalid version not set");
    }

    // Ported: "should handle lock file parsing and extracting errors" — gleam/extract.spec.ts line 193
    #[test]
    fn gleam_extract_invalid_lock_toml() {
        let deps = extract_with_lock(GLEAM_TOML, Some("invalid"));
        assert!(!deps.is_empty(), "expected deps");
        let foo = deps.iter().find(|d| d.name == "foo").unwrap();
        assert!(foo.locked_version.is_none(), "parse error: no locked version");
    }

    const LOCK_FILE: &str = r#"packages = [
  { name = "foo", version = "1.0.4", build_tools = ["gleam"], requirements = ["bar"], otp_app = "foo", source = "hex", outer_checksum = "ABC" },
  { name = "bar", version = "2.1.0", build_tools = ["rebar3"], requirements = [], otp_app = "bar", source = "hex", outer_checksum = "DEF" },
]

[requirements]
foo = { version = ">= 1.0.0 and < 2.0.0" }
"#;

    // Ported: "returns null for missing lock file" — modules/manager/gleam/locked-version.spec.ts line 19
    #[test]
    fn extract_versions_missing_file_returns_none() {
        assert!(extract_gleam_lock_file_versions_opt(None).is_none());
    }

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

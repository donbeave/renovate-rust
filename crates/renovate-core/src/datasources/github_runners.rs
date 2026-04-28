//! GitHub-hosted runner versions datasource.
//!
//! A static, offline datasource that maps runner names to their known version
//! history.  No network call is needed — the table is maintained in sync with
//! the upstream Renovate reference:
//! `lib/modules/datasource/github-runners/index.ts`.
//!
//! ## Runner name format
//!
//! GitHub Actions `runs-on:` values follow the pattern `{name}-{version}`:
//!
//! | Input | Runner name | Version |
//! |---|---|---|
//! | `ubuntu-22.04` | `ubuntu` | `22.04` |
//! | `macos-14-xlarge` | `macos` | `14-xlarge` |
//! | `windows-2022` | `windows` | `2022` |
//!
//! ## Update semantics
//!
//! An update is only offered when the latest **stable**, **non-deprecated**
//! version with the *same variant suffix* differs from the current version.
//! The variant suffix is the portion of the version string after the leading
//! numeric segment (e.g., `-arm`, `-xlarge`, `-large`, or empty).
//!
//! This means `ubuntu-22.04` may update to `ubuntu-24.04`, but `ubuntu-22.04-arm`
//! only updates within the `-arm` variant track.

/// A single runner version record.
#[derive(Debug)]
pub struct RunnerVersion {
    pub version: &'static str,
    /// `false` when the runner is still in beta / preview.
    pub stable: bool,
    /// `true` when GitHub no longer offers this runner.
    pub deprecated: bool,
}

/// All known runner releases, newest-first within each name.
///
/// Ported from `GithubRunnersDatasource.releases` in
/// `lib/modules/datasource/github-runners/index.ts`.
static RUNNERS: &[(&str, &[RunnerVersion])] = &[
    (
        "ubuntu",
        &[
            RunnerVersion {
                version: "24.04",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "24.04-arm",
                stable: false,
                deprecated: false,
            },
            RunnerVersion {
                version: "22.04",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "22.04-arm",
                stable: false,
                deprecated: false,
            },
            RunnerVersion {
                version: "20.04",
                stable: true,
                deprecated: true,
            },
            RunnerVersion {
                version: "18.04",
                stable: true,
                deprecated: true,
            },
            RunnerVersion {
                version: "16.04",
                stable: true,
                deprecated: true,
            },
        ],
    ),
    (
        "macos",
        &[
            RunnerVersion {
                version: "26",
                stable: false,
                deprecated: false,
            },
            RunnerVersion {
                version: "26-xlarge",
                stable: false,
                deprecated: false,
            },
            RunnerVersion {
                version: "15",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "15-large",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "15-xlarge",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "14",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "14-large",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "14-xlarge",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "13",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "13-large",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "13-xlarge",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "12-large",
                stable: true,
                deprecated: true,
            },
            RunnerVersion {
                version: "12",
                stable: true,
                deprecated: true,
            },
            RunnerVersion {
                version: "11",
                stable: true,
                deprecated: true,
            },
            RunnerVersion {
                version: "10.15",
                stable: true,
                deprecated: true,
            },
        ],
    ),
    (
        "windows",
        &[
            RunnerVersion {
                version: "2025",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "2022",
                stable: true,
                deprecated: false,
            },
            RunnerVersion {
                version: "2019",
                stable: true,
                deprecated: true,
            },
            RunnerVersion {
                version: "2016",
                stable: true,
                deprecated: true,
            },
        ],
    ),
];

/// Result of a runner version lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerUpdateSummary {
    pub current: String,
    pub latest: Option<String>,
    pub update_available: bool,
    pub deprecated: bool,
}

/// Return `true` when `name` is a known runner name and `version` is a known
/// version for it (matches `GithubRunnersDatasource.isValidRunner`).
pub fn is_valid_runner(name: &str, version: &str) -> bool {
    RUNNERS
        .iter()
        .find(|(n, _)| *n == name)
        .is_some_and(|(_, versions)| versions.iter().any(|v| v.version == version))
}

/// Extract the non-numeric variant suffix from a version string.
///
/// - `"22.04"` → `""`
/// - `"22.04-arm"` → `"-arm"`
/// - `"14-xlarge"` → `"-xlarge"`
/// - `"15-large"` → `"-large"`
fn variant_suffix(version: &str) -> &str {
    let end = version
        .find(|c: char| !c.is_ascii_digit() && c != '.')
        .unwrap_or(version.len());
    &version[end..]
}

/// Find the latest stable, non-deprecated runner version for the given runner
/// name that shares the same variant suffix as `current_version`.
///
/// Returns `None` when `name` is unknown or no stable non-deprecated version
/// with the same suffix exists.
pub fn latest_stable(name: &str, current_version: &str) -> Option<&'static str> {
    let versions = RUNNERS.iter().find(|(n, _)| *n == name)?.1;
    let suffix = variant_suffix(current_version);
    versions
        .iter()
        .find(|v| v.stable && !v.deprecated && variant_suffix(v.version) == suffix)
        .map(|v| v.version)
}

/// Compute an update summary for a runner dep.
pub fn update_summary(name: &str, current_version: &str) -> RunnerUpdateSummary {
    let deprecated = RUNNERS
        .iter()
        .find(|(n, _)| *n == name)
        .and_then(|(_, versions)| versions.iter().find(|v| v.version == current_version))
        .is_some_and(|v| v.deprecated);

    let latest = latest_stable(name, current_version);
    let update_available = latest.is_some_and(|l| l != current_version);

    RunnerUpdateSummary {
        current: current_version.to_owned(),
        latest: latest.map(|s| s.to_owned()),
        update_available,
        deprecated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_suffix_empty_for_plain_version() {
        assert_eq!(variant_suffix("22.04"), "");
        assert_eq!(variant_suffix("2022"), "");
        assert_eq!(variant_suffix("15"), "");
    }

    #[test]
    fn variant_suffix_captures_non_numeric_tail() {
        assert_eq!(variant_suffix("22.04-arm"), "-arm");
        assert_eq!(variant_suffix("14-xlarge"), "-xlarge");
        assert_eq!(variant_suffix("15-large"), "-large");
    }

    #[test]
    fn is_valid_runner_known_versions() {
        assert!(is_valid_runner("ubuntu", "22.04"));
        assert!(is_valid_runner("ubuntu", "24.04"));
        assert!(is_valid_runner("macos", "15"));
        assert!(is_valid_runner("macos", "14-xlarge"));
        assert!(is_valid_runner("windows", "2022"));
    }

    #[test]
    fn is_valid_runner_unknown_name() {
        assert!(!is_valid_runner("debian", "12"));
        assert!(!is_valid_runner("", "22.04"));
    }

    #[test]
    fn is_valid_runner_unknown_version() {
        assert!(!is_valid_runner("ubuntu", "99.99"));
        assert!(!is_valid_runner("macos", "latest"));
    }

    #[test]
    fn latest_stable_ubuntu_plain() {
        // ubuntu-22.04 → latest plain ubuntu is 24.04
        assert_eq!(latest_stable("ubuntu", "22.04"), Some("24.04"));
    }

    #[test]
    fn latest_stable_ubuntu_arm() {
        // ubuntu-22.04-arm → latest -arm ubuntu is 24.04-arm (currently unstable, so falls back)
        // 24.04-arm is not stable → none stable in -arm track yet
        assert_eq!(latest_stable("ubuntu", "22.04-arm"), None);
    }

    #[test]
    fn latest_stable_macos_xlarge() {
        // macos-14-xlarge → latest xlarge macos
        assert_eq!(latest_stable("macos", "14-xlarge"), Some("15-xlarge"));
    }

    #[test]
    fn latest_stable_macos_plain() {
        assert_eq!(latest_stable("macos", "14"), Some("15"));
    }

    #[test]
    fn latest_stable_windows() {
        assert_eq!(latest_stable("windows", "2022"), Some("2025"));
    }

    #[test]
    fn latest_stable_already_latest() {
        assert_eq!(latest_stable("ubuntu", "24.04"), Some("24.04"));
    }

    #[test]
    fn update_summary_outdated() {
        let s = update_summary("ubuntu", "22.04");
        assert!(s.update_available);
        assert_eq!(s.latest, Some("24.04".into()));
        assert!(!s.deprecated);
    }

    #[test]
    fn update_summary_up_to_date() {
        let s = update_summary("ubuntu", "24.04");
        assert!(!s.update_available);
        assert_eq!(s.latest, Some("24.04".into()));
    }

    #[test]
    fn update_summary_deprecated() {
        let s = update_summary("ubuntu", "20.04");
        assert!(s.deprecated);
        assert_eq!(s.latest, Some("24.04".into()));
        assert!(s.update_available);
    }

    #[test]
    fn update_summary_windows_outdated() {
        let s = update_summary("windows", "2019");
        assert!(s.update_available);
        assert_eq!(s.latest, Some("2025".into()));
        assert!(s.deprecated);
    }
}

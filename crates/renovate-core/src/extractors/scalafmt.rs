//! Scalafmt `.scalafmt.conf` version extractor.
//!
//! Reads the `version = "x.y.z"` setting from Scalafmt configuration files.
//! The version maps to GitHub Releases for `scalameta/scalafmt`.
//!
//! Renovate reference:
//! - `lib/modules/manager/scalafmt/extract.ts`
//! - Pattern: `/(^|/)\.scalafmt\.conf$/`
//!
//! ## Supported form
//!
//! ```hocon
//! version = "3.7.14"
//! maxColumn = 100
//! ```

/// The GitHub repository for Scalafmt releases.
pub const SCALAFMT_REPO: &str = "scalameta/scalafmt";

/// A single extracted Scalafmt version dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalafmtDep {
    /// Current Scalafmt version (e.g. `"3.7.14"`).
    pub version: String,
}

/// Parse `.scalafmt.conf` and extract the `version` setting.
pub fn extract(content: &str) -> Option<ScalafmtDep> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(val) = trimmed
            .strip_prefix("version")
            .and_then(|s| s.trim_start().strip_prefix('='))
        {
            let version = val.trim().trim_matches('"').trim_matches('\'').trim();
            if !version.is_empty() && version.contains('.') {
                return Some(ScalafmtDep {
                    version: version.to_owned(),
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_version() {
        let content = "version = \"3.7.14\"\nmaxColumn = 100\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "3.7.14");
    }

    #[test]
    fn version_without_quotes() {
        let content = "version = 3.7.14\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "3.7.14");
    }

    #[test]
    fn no_version_returns_none() {
        assert!(extract("maxColumn = 100\n").is_none());
    }

    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }
}

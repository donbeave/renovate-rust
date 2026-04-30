//! Heroku/Render `runtime.txt` Python version extractor.
//!
//! Parses `runtime.txt` files that specify the Python runtime version used by
//! Heroku, Render, and similar PaaS platforms.
//!
//! Renovate reference:
//! - `lib/modules/manager/runtime-version/extract.ts`
//! - Pattern: `(^|/)runtime\.txt$`
//! - Datasource: GitHub Releases on `python/cpython` (upstream uses DockerDatasource)
//!
//! ## File format
//!
//! ```text
//! python-3.11.0
//! ```
//!
//! Only exact 3-part semver versions are extracted (`3.11.0`), not ranges or
//! partial versions like `3.11`.

use std::sync::LazyLock;

use regex::Regex;

/// Matches `python-X.Y.Z` (exactly three numeric components).
static PYTHON_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^python-(\d+\.\d+\.\d+)\s*$").unwrap());

/// A single Python runtime dep extracted from `runtime.txt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeVersionDep {
    /// Always `"python"`.
    pub dep_name: String,
    /// Version string, e.g. `"3.11.0"`.
    pub current_value: String,
}

/// Extract the Python version from a `runtime.txt` file.
///
/// Returns `None` if no `python-X.Y.Z` line is found.
pub fn extract(content: &str) -> Option<RuntimeVersionDep> {
    let cap = PYTHON_RE.captures(content)?;
    Some(RuntimeVersionDep {
        dep_name: "python".to_owned(),
        current_value: cap[1].to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns a result - python" — runtime-version/extract.spec.ts line 5
    #[test]
    fn extracts_python_version() {
        let dep = extract("python-3.11.0\n").unwrap();
        assert_eq!(dep.dep_name, "python");
        assert_eq!(dep.current_value, "3.11.0");
    }

    #[test]
    fn extracts_with_trailing_newline() {
        let dep = extract("python-3.12.1\r\n").unwrap();
        assert_eq!(dep.current_value, "3.12.1");
    }

    #[test]
    fn ignores_partial_version() {
        assert!(extract("python-3.11\n").is_none());
    }

    // Ported: "returns no result" — runtime-version/extract.spec.ts line 16
    #[test]
    fn returns_none_for_empty() {
        assert!(extract("").is_none());
        assert!(extract("nodejs-18.0.0\n").is_none());
    }
}

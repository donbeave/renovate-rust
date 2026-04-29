//! Batect wrapper script version extractor.
//!
//! Finds the `VERSION="x.y.z"` line in the Batect wrapper shell script
//! and returns it as a GitHub Releases dependency.
//!
//! Renovate reference:
//! - `lib/modules/manager/batect-wrapper/extract.ts`
//! - Pattern: `/(^|/)batect$/`
//! - Datasource: GitHub Releases (`batect/batect`)

use std::sync::LazyLock;

use regex::Regex;

pub const BATECT_REPO: &str = "batect/batect";

/// The extracted Batect wrapper version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatectWrapperDep {
    pub version: String,
}

static VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s+VERSION="([^"]+)""#).unwrap());

/// Extract the Batect version from a wrapper script.
pub fn extract(content: &str) -> Option<BatectWrapperDep> {
    let cap = VERSION_RE.captures(content)?;
    let version = cap[1].trim().to_owned();
    if version.is_empty() {
        return None;
    }
    Some(BatectWrapperDep { version })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"#!/usr/bin/env bash
set -euo pipefail

# batect wrapper script version 1.2.3

    VERSION="0.63.4"

DOWNLOAD_URL="https://github.com/batect/batect/releases/download/${VERSION}/batect"
"#;

    // Ported: "extracts the current version from a valid wrapper script" — batect-wrapper/extract.spec.ts line 17
    #[test]
    fn extracts_version() {
        let dep = extract(SAMPLE).unwrap();
        assert_eq!(dep.version, "0.63.4");
    }

    // Ported: "returns null for file without version information" — batect-wrapper/extract.spec.ts line 13
    #[test]
    fn no_version_line_returns_none() {
        assert!(extract("#!/bin/bash\necho hello\n").is_none());
    }

    // Ported: "returns null for empty wrapper file" — batect-wrapper/extract.spec.ts line 9
    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }

    // Ported: "returns the first version from a wrapper script with multiple versions" — batect-wrapper/extract.spec.ts line 31
    #[test]
    fn multiple_version_lines_uses_first() {
        let content = "#!/usr/bin/env bash\n{\n    VERSION=\"0.60.1\"\n    VERSION=\"0.63.0\"\n}";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "0.60.1");
    }
}

//! Mint `Mintfile` dependency extractor.
//!
//! Parses Swift tool dependency files managed by Mint. Each non-comment line
//! follows the format `owner/repo@version`.
//!
//! Renovate reference:
//! - `lib/modules/manager/mint/extract.ts`
//! - Pattern: `/(^|/)Mintfile$/`
//!
//! ## Supported form
//!
//! ```text
//! yonaskolb/XcodeGen@2.38.0
//! nicklockwood/SwiftFormat@0.52.8
//! # comment line
//! ```

/// A single extracted Mint dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintDep {
    /// GitHub `owner/repo` (e.g. `"yonaskolb/XcodeGen"`).
    pub repo: String,
    /// Version tag (e.g. `"2.38.0"`).
    pub version: String,
}

/// Parse a `Mintfile` and extract all dependencies.
pub fn extract(content: &str) -> Vec<MintDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        // Strip inline comments.
        let line = raw.split('#').next().unwrap_or(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some((repo, version)) = line.split_once('@') {
            let repo = repo.trim();
            let version = version.trim();
            if !repo.is_empty() && !version.is_empty() {
                out.push(MintDep {
                    repo: repo.to_owned(),
                    version: version.to_owned(),
                });
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_deps() {
        let content = "yonaskolb/XcodeGen@2.38.0\nnicklockwood/SwiftFormat@0.52.8\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].repo, "yonaskolb/XcodeGen");
        assert_eq!(deps[0].version, "2.38.0");
        assert_eq!(deps[1].repo, "nicklockwood/SwiftFormat");
    }

    #[test]
    fn comment_lines_skipped() {
        let content = "# tool list\nyonaskolb/XcodeGen@2.38.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn inline_comment_stripped() {
        let content = "yonaskolb/XcodeGen@2.38.0 # pinned version\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, "2.38.0");
    }

    #[test]
    fn line_without_at_skipped() {
        let content = "yonaskolb/XcodeGen\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}

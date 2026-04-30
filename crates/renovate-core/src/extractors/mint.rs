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

/// Why a Mint dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintSkipReason {
    /// No version specified (`owner/repo` without `@version`).
    UnspecifiedVersion,
}

/// A single extracted Mint dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintDep {
    /// GitHub `owner/repo` (e.g. `"yonaskolb/XcodeGen"`).
    pub repo: String,
    /// Version tag (e.g. `"2.38.0"`), empty when unspecified.
    pub version: String,
    /// Set when no version lookup should be performed.
    pub skip_reason: Option<MintSkipReason>,
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
            let version = version.trim().split('@').next().unwrap_or("").trim();
            if !repo.is_empty() {
                if version.is_empty() {
                    out.push(MintDep {
                        repo: repo.to_owned(),
                        version: String::new(),
                        skip_reason: Some(MintSkipReason::UnspecifiedVersion),
                    });
                } else {
                    out.push(MintDep {
                        repo: repo.to_owned(),
                        version: version.to_owned(),
                        skip_reason: None,
                    });
                }
            }
        } else {
            // No `@` — no version specified.
            let repo = line.trim();
            if !repo.is_empty() && repo.contains('/') {
                out.push(MintDep {
                    repo: repo.to_owned(),
                    version: String::new(),
                    skip_reason: Some(MintSkipReason::UnspecifiedVersion),
                });
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns null for empty" — mint/extract.spec.ts line 6
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "Mintfile With Version Description" — mint/extract.spec.ts line 10
    #[test]
    fn extracts_deps_with_version() {
        let content = "SwiftGen/SwiftGen@6.6.1\nyonaskolb/xcodegen@2.30.0\nrealm/SwiftLint @ 0.48.0\n#realm/SwiftLint @ 0.48.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(deps.iter().any(|d| d.repo == "SwiftGen/SwiftGen"
            && d.version == "6.6.1"
            && d.skip_reason.is_none()));
        assert!(
            deps.iter()
                .any(|d| d.repo == "yonaskolb/xcodegen" && d.version == "2.30.0")
        );
        assert!(
            deps.iter()
                .any(|d| d.repo == "realm/SwiftLint" && d.version == "0.48.0")
        );
    }

    // Ported: "Mintfile Without Version Description" — mint/extract.spec.ts line 41
    #[test]
    fn extracts_deps_without_version_as_skipped() {
        let content = "yonaskolb/xcodegen\nrealm/SwiftLint\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(MintSkipReason::UnspecifiedVersion))
        );
    }

    // Ported: "Complex Mintfile" — mint/extract.spec.ts line 61
    #[test]
    fn complex_mintfile_mixed() {
        let content = "SwiftGen/SwiftGen@6.6.1\nyonaskolb/xcodegen\nrealm/SwiftLint @ 0.48.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        let xcode = deps
            .iter()
            .find(|d| d.repo == "yonaskolb/xcodegen")
            .unwrap();
        assert_eq!(xcode.skip_reason, Some(MintSkipReason::UnspecifiedVersion));
        let swift = deps.iter().find(|d| d.repo == "SwiftGen/SwiftGen").unwrap();
        assert!(swift.skip_reason.is_none());
    }

    // Ported: "Mintfile Includes Commented Out" — mint/extract.spec.ts line 86
    #[test]
    fn comment_lines_skipped() {
        let content = "SwiftGen/SwiftGen@6.6.1\n\nyonaskolb/xcodegen\n#yonaskolb/xcodegen\nrealm/SwiftLint@0.48.0 #commented out\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter()
                .any(|d| d.repo == "SwiftGen/SwiftGen" && d.version == "6.6.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.repo == "yonaskolb/xcodegen" && d.skip_reason.is_some())
        );
        assert!(
            deps.iter()
                .any(|d| d.repo == "realm/SwiftLint" && d.version == "0.48.0")
        );
    }

    #[test]
    fn inline_comment_stripped() {
        let content = "yonaskolb/XcodeGen@2.38.0 # pinned version\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, "2.38.0");
    }
}

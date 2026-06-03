//! Unity3D `ProjectVersion.txt` version extractor.
//!
//! Parses `ProjectSettings/ProjectVersion.txt` files and extracts the
//! Unity Editor version for update checking.
//!
//! Renovate reference:
//! - `lib/modules/manager/unity3d/extract.ts`
//! - Pattern: `(^|/)ProjectSettings/ProjectVersion\.txt$`
//! - Datasource: `unity3d`
//!
//! ## File format
//!
//! ```text
//! m_EditorVersion: 2022.3.10f1
//! m_EditorVersionWithRevision: 2022.3.10f1 (ff3792e53c62)
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Whether the extracted version includes the short commit revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unity3dVersionKind {
    /// Plain version (`m_EditorVersion`).
    Plain,
    /// Version with short revision hash (`m_EditorVersionWithRevision`).
    WithRevision,
}

/// A Unity3D Editor version dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unity3dDep {
    pub current_value: String,
    pub kind: Unity3dVersionKind,
}

/// Matches `m_EditorVersion: 2022.3.10f1` or
/// `m_EditorVersionWithRevision: 2022.3.10f1 (ff3792e53c62)`.
static VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^m_EditorVersion:\s*(.*?)\s*$").unwrap());

/// Matches `m_EditorVersionWithRevision:` specifically.
static WITH_REV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^m_EditorVersionWithRevision:\s*(.+?)\s*$").unwrap());

/// Extract the Unity3D Editor versions from a `ProjectVersion.txt` file.
///
/// Returns an empty vector if no version field is found.
pub fn extract(content: &str) -> Vec<Unity3dDep> {
    let mut deps = Vec::new();

    if let Some(cap) = VERSION_RE.captures(content) {
        let current_value = cap[1].trim();
        if !current_value.is_empty() {
            deps.push(Unity3dDep {
                current_value: current_value.to_owned(),
                kind: Unity3dVersionKind::Plain,
            });
        }
    }

    if let Some(cap) = WITH_REV_RE.captures(content) {
        let current_value = cap[1].trim();
        if !current_value.is_empty() {
            deps.push(Unity3dDep {
                current_value: current_value.to_owned(),
                kind: Unity3dVersionKind::WithRevision,
            });
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "handles $packageName" — lib/modules/manager/unity3d/extract.spec.ts line 14
    #[test]
    fn extracts_plain_version() {
        let deps = extract("m_EditorVersion: 2022.3.19f1\n");
        assert_eq!(
            deps,
            vec![Unity3dDep {
                current_value: "2022.3.19f1".to_owned(),
                kind: Unity3dVersionKind::Plain,
            }]
        );
    }

    // Ported: "handles $packageName" — lib/modules/manager/unity3d/extract.spec.ts line 14
    #[test]
    fn extracts_with_revision_version() {
        let deps = extract("m_EditorVersionWithRevision: 2022.3.19f1 (30acc77e9b6b)\n");
        assert_eq!(
            deps,
            vec![Unity3dDep {
                current_value: "2022.3.19f1 (30acc77e9b6b)".to_owned(),
                kind: Unity3dVersionKind::WithRevision,
            }]
        );
    }

    // Ported: "handles no version" — lib/modules/manager/unity3d/extract.spec.ts line 5
    #[test]
    fn returns_none_for_empty() {
        assert!(extract("").is_empty());
        assert!(extract("m_EditorVersion: ").is_empty());
        assert!(extract("m_EditorVersionWithRevision: ").is_empty());
        assert!(extract("something: else\n").is_empty());
    }

    // Ported: "handles $type versions" — lib/modules/manager/unity3d/extract.spec.ts line 39
    #[test]
    fn extracts_alpha_beta_and_stable_versions_with_revisions() {
        let cases = [
            (
                "m_EditorVersion: 2022.3.0a1\nm_EditorVersionWithRevision: 2022.3.0a1 (244b723c30a6)\n",
                "2022.3.0a1",
                "2022.3.0a1 (244b723c30a6)",
            ),
            (
                "m_EditorVersion: 2023.3.0b5\nm_EditorVersionWithRevision: 2023.3.0b5 (30acc77e9b6b)\n",
                "2023.3.0b5",
                "2023.3.0b5 (30acc77e9b6b)",
            ),
            (
                "m_EditorVersion: 2021.3.35f1\nm_EditorVersionWithRevision: 2021.3.35f1 (122a674b12f3)\n",
                "2021.3.35f1",
                "2021.3.35f1 (122a674b12f3)",
            ),
        ];

        for (content, version, version_with_revision) in cases {
            assert_eq!(
                extract(content),
                vec![
                    Unity3dDep {
                        current_value: version.to_owned(),
                        kind: Unity3dVersionKind::Plain,
                    },
                    Unity3dDep {
                        current_value: version_with_revision.to_owned(),
                        kind: Unity3dVersionKind::WithRevision,
                    },
                ]
            );
        }
    }
}

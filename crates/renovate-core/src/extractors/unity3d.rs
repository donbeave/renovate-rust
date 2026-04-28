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

/// A single Unity3D Editor version dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unity3dDep {
    pub current_value: String,
    pub kind: Unity3dVersionKind,
}

/// Matches `m_EditorVersion: 2022.3.10f1` or
/// `m_EditorVersionWithRevision: 2022.3.10f1 (ff3792e53c62)`.
static VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^m_EditorVersion(?:WithRevision)?:\s*(.+?)\s*$").unwrap());

/// Matches `m_EditorVersionWithRevision:` specifically.
static WITH_REV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^m_EditorVersionWithRevision:\s*(.+?)\s*$").unwrap());

/// Extract the Unity3D Editor version from a `ProjectVersion.txt` file.
///
/// Returns `None` if no version field is found.
pub fn extract(content: &str) -> Option<Unity3dDep> {
    // Prefer `m_EditorVersionWithRevision` if present.
    if let Some(cap) = WITH_REV_RE.captures(content) {
        return Some(Unity3dDep {
            current_value: cap[1].to_owned(),
            kind: Unity3dVersionKind::WithRevision,
        });
    }

    // Fall back to plain `m_EditorVersion`.
    if let Some(cap) = VERSION_RE.captures(content) {
        return Some(Unity3dDep {
            current_value: cap[1].to_owned(),
            kind: Unity3dVersionKind::Plain,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_plain_version() {
        let content = "m_EditorVersion: 2022.3.10f1\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "2022.3.10f1");
        assert_eq!(dep.kind, Unity3dVersionKind::Plain);
    }

    #[test]
    fn prefers_with_revision_version() {
        let content = "m_EditorVersion: 2022.3.10f1\nm_EditorVersionWithRevision: 2022.3.10f1 (ff3792e53c62)\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "2022.3.10f1 (ff3792e53c62)");
        assert_eq!(dep.kind, Unity3dVersionKind::WithRevision);
    }

    #[test]
    fn returns_none_for_empty() {
        assert!(extract("").is_none());
        assert!(extract("something: else\n").is_none());
    }
}

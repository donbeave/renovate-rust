//! Changelog retrieval.
//!
//! Mirrors `lib/workers/repository/changelog/index.ts`.
//! @parity lib/workers/repository/changelog/index.ts partial — embedChangelog + embedChangelogs (stage filter on fetchChangeLogs, pre-provided changelogContent synthetic path, delegation to get / skip if logJSON already set) implemented using EmbeddableUpgrade + the ChangeLog* types. The actual getChangeLogJSON (release notes fetch), wiring into BranchUpgrade during branchify/update/pr, and full PR body rendering live in other (pending) repository/update/pr/changelog and branch modules.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogSource {
    pub source_url: Option<String>,
    pub source_directory: Option<String>,
    pub base_version: Option<String>,
    pub new_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogResult {
    pub project: Option<ChangeLogSource>,
    pub has_release_notes: bool,
    pub releases: Vec<ChangeLogRelease>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogRelease {
    pub version: Option<String>,
    pub date: Option<String>,
    pub git_ref: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub compare_url: Option<String>,
}

/// Minimal container for the embed logic (fields relevant to embedChangelog/embedChangelogs from BranchUpgradeConfig).
/// The real BranchUpgrade/processing upgrades will map to this (or share fields) when the calling code is ported.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmbeddableUpgrade {
    pub fetch_change_logs: Option<String>,
    pub log_json: Option<ChangeLogResult>,
    pub changelog_content: Option<String>,
    pub package_name: Option<String>,
    pub dep_name: Option<String>,
    pub source_url: Option<String>,
    pub source_directory: Option<String>,
    pub new_version: Option<String>,
}

pub fn get_change_log(source: &ChangeLogSource) -> ChangeLogResult {
    ChangeLogResult {
        project: Some(source.clone()),
        has_release_notes: false,
        releases: Vec::new(),
    }
}

/// Mirrors embedChangelog from lib/workers/repository/changelog/index.ts.
/// If log_json already set (including the "null" case passed as Some), skip.
/// If changelogContent provided, build synthetic logJSON (hasReleaseNotes + releaseNotes body).
/// Otherwise delegate to get_change_log (the analogue of getChangeLogJSON for this unit).
pub fn embed_changelog(upgrade: &mut EmbeddableUpgrade) {
    if upgrade.log_json.is_some() {
        // logJSON !== undefined in TS (even explicit null skips re-fetch)
        return;
    }
    if let Some(ref content) = upgrade.changelog_content {
        upgrade.log_json = Some(ChangeLogResult {
            has_release_notes: true,
            project: Some(ChangeLogSource {
                source_url: upgrade.source_url.clone(),
                source_directory: upgrade.source_directory.clone(),
                ..Default::default()
            }),
            releases: vec![ChangeLogRelease {
                body: Some(content.clone()),
                version: upgrade.new_version.clone(),
                ..Default::default()
            }],
        });
    } else {
        let source = ChangeLogSource {
            source_url: upgrade.source_url.clone(),
            source_directory: upgrade.source_directory.clone(),
            new_version: upgrade.new_version.clone(),
            ..Default::default()
        };
        upgrade.log_json = Some(get_change_log(&source));
    }
}

/// Mirrors embedChangelogs from lib/workers/repository/changelog/index.ts.
/// Filters upgrades to only those whose fetchChangeLogs matches the stage ('pr' | 'branch'; 'off' never matches).
/// Then runs embed for the filtered (concurrency 10 in TS via p.map; sequential here is equivalent for observable result since get is sync stub).
pub fn embed_changelogs(upgrades: &mut [EmbeddableUpgrade], stage: &str) {
    for upgrade in upgrades.iter_mut() {
        if upgrade.fetch_change_logs.as_deref() == Some(stage) {
            embed_changelog(upgrade);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_log_source_default() {
        let s = ChangeLogSource::default();
        assert!(s.source_url.is_none());
        assert!(s.base_version.is_none());
    }

    #[test]
    fn change_log_result_default() {
        let r = ChangeLogResult::default();
        assert!(r.project.is_none());
        assert!(!r.has_release_notes);
        assert!(r.releases.is_empty());
    }

    #[test]
    fn change_log_release_default() {
        let r = ChangeLogRelease::default();
        assert!(r.version.is_none());
    }

    #[test]
    fn get_change_log_returns_result() {
        let source = ChangeLogSource {
            source_url: Some("https://github.com/lodash/lodash".into()),
            base_version: Some("4.17.0".into()),
            new_version: Some("4.17.21".into()),
            ..Default::default()
        };
        let result = get_change_log(&source);
        assert!(result.project.is_some());
        assert!(!result.has_release_notes);
    }

    #[test]
    fn change_log_source_serialization_roundtrip() {
        let s = ChangeLogSource {
            source_url: Some("https://github.com/test/test".into()),
            source_directory: None,
            base_version: Some("1.0.0".into()),
            new_version: Some("2.0.0".into()),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: ChangeLogSource = serde_json::from_str(&json).unwrap();
        assert_eq!(back.source_url, Some("https://github.com/test/test".into()));
    }

    #[test]
    fn change_log_release_serialization_roundtrip() {
        let r = ChangeLogRelease {
            version: Some("1.0.0".into()),
            date: Some("2024-01-01".into()),
            body: Some("Bug fixes".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ChangeLogRelease = serde_json::from_str(&json).unwrap();
        assert_eq!(back.version, Some("1.0.0".into()));
    }

    // Ported: "only fetches changelogs for upgrades whose fetchChangeLogs matches the stage name" — lib/workers/repository/changelog/index.spec.ts line 55
    #[test]
    fn embed_changelogs_only_fetches_for_matching_stage() {
        let mut upgrades = vec![
            EmbeddableUpgrade {
                fetch_change_logs: Some("pr".into()),
                ..Default::default()
            },
            EmbeddableUpgrade {
                fetch_change_logs: Some("branch".into()),
                ..Default::default()
            },
            EmbeddableUpgrade {
                fetch_change_logs: Some("off".into()),
                ..Default::default()
            },
            EmbeddableUpgrade {
                fetch_change_logs: Some("pr".into()),
                changelog_content: Some("testContent".into()),
                source_url: Some("https://example.com".into()),
                new_version: Some("2.0.0".into()),
                ..Default::default()
            },
        ];
        embed_changelogs(&mut upgrades, "branch");
        // only the 'branch' one (and not 'off' or 'pr') should have been processed by embed
        assert!(upgrades[1].log_json.is_some());
        assert!(upgrades[0].log_json.is_none());
        assert!(upgrades[2].log_json.is_none());
        // the content short-circuit also works for matching stage (tested via pr in other cases)
        // now test content path + already-set skip for the main embedChangelogs behavior
        let mut upgrades2 = vec![
            EmbeddableUpgrade {
                fetch_change_logs: Some("pr".into()),
                log_json: Some(ChangeLogResult::default()), // pre-set (simulates explicit null case in TS)
                ..Default::default()
            },
            EmbeddableUpgrade {
                fetch_change_logs: Some("pr".into()),
                ..Default::default()
            },
            EmbeddableUpgrade {
                fetch_change_logs: Some("pr".into()),
                changelog_content: Some("testContent".into()),
                source_url: Some("https://ex".into()),
                new_version: Some("1.2.3".into()),
                ..Default::default()
            },
        ];
        embed_changelogs(&mut upgrades2, "pr");
        assert!(upgrades2[0].log_json.is_some()); // kept pre-set
        assert!(upgrades2[1].log_json.is_some()); // fetched via get
        let content_log = upgrades2[2].log_json.as_ref().unwrap();
        assert!(content_log.has_release_notes);
        assert_eq!(content_log.releases.len(), 1);
        assert_eq!(content_log.releases[0].body, Some("testContent".into()));
    }
}

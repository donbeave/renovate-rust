//! Internal presets registry.
//!
//! Renovate reference: `lib/config/presets/internal/index.ts`.
//!
//! Provides built-in presets grouped by category (default, config, docker,
//! group, helpers, monorepo, packages, preview, replacements, schedule,
//! security, workarounds, etc.).

use std::collections::BTreeMap;
use std::sync::LazyLock;

use serde_json::{Value, json};

type PresetMap = BTreeMap<&'static str, Value>;

static INTERNAL_PRESETS: LazyLock<BTreeMap<&str, PresetMap>> = LazyLock::new(|| {
    let mut groups: BTreeMap<&str, PresetMap> = BTreeMap::new();

    groups.insert("default", default_presets());
    groups.insert("config", config_presets());
    groups.insert("docker", docker_presets());
    groups.insert("group", group_presets());
    groups.insert("helpers", helpers_presets());
    groups.insert("monorepo", monorepo_presets());
    groups.insert("packages", packages_presets());
    groups.insert("preview", preview_presets());
    groups.insert("schedule", schedule_presets());
    groups.insert("security", security_presets());
    groups.insert("workarounds", workarounds_presets());
    groups.insert("replacements", replacements_presets());
    groups.insert("mergeConfidence", merge_confidence_presets());
    groups.insert("customManagers", custom_managers_presets());
    groups.insert("abandonments", abandonments_presets());
    groups.insert("global", global_presets());

    groups
});

fn default_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("approveMajorUpdates", json!({"dependencyDashboardApproval": true, "matchUpdateTypes": ["major"]}));
    m.insert("automergeDigest", json!({"automerge": true, "matchUpdateTypes": ["digest"]}));
    m.insert("automergePatch", json!({"automerge": true, "matchUpdateTypes": ["patch"]}));
    m.insert("automergeBranch", json!({"automerge": true, "automergeType": "branch"}));
    m.insert("automergeMajor", json!({"automerge": true, "matchUpdateTypes": ["major"]}));
    m.insert("automergeMinor", json!({"automerge": true, "matchUpdateTypes": ["minor"]}));
    m.insert("autodetectPinVersions", json!({}));
    m.insert("automergeRequireAllStatusChecks", json!({"ignoreTests": false, "requiredStatusChecks": null}));
    m.insert("dependencyDashboard", json!({"dependencyDashboard": true}));
    m.insert("dependencyDashboardApproval", json!({"dependencyDashboardApproval": true}));
    m.insert("disableDependencyDashboard", json!({"dependencyDashboard": false}));
    m.insert("disableDevDependencies", json!({"matchDepTypes": ["devDependencies"], "enabled": false}));
    m.insert("disableDigestUpdates", json!({"matchUpdateTypes": ["digest"], "enabled": false}));
    m.insert("disableMajorUpdates", json!({"matchUpdateTypes": ["major"], "enabled": false}));
    m.insert("disablePeerDependencies", json!({"matchDepTypes": ["peerDependencies"], "enabled": false}));
    m.insert("disablePrControls", json!({"dependencyDashboardApproval": false, "dependencyDashboardAutoClose": false, "prCreation": "immediate"}));
    m.insert("doNotPinPackage", json!({}));
    m.insert("enablePreCommit", json!({"customManagers": [{"customType": "regex", "managerFilePatterns": ["/(^|/)\\.pre-commit-config\\.yaml$/"], "matchStrings": ["  - repo: (?<repo>https?://[^\\s]+)\\n    rev: (?<currentValue>[^\\s]+)"], "datasourceTemplate": "git-tags", "depNameTemplate": "{{{repo}}}"}]}));
    m.insert("enableRenovate", json!({"enabled": true}));
    m.insert("enableVulnerabilityAlerts", json!({"vulnerabilityAlerts": {"enabled": true}}));
    m.insert("gitSignOff", json!({"commitBody": "Signed-off-by: {{{gitAuthor}}}"}));
    m.insert("ignoreModulesAndTests", json!({"ignorePaths": ["**/node_modules/**", "**/bower_components/**", "**/test/**", "**/tests/**", "**/__tests__/**", "**/spec/**", "**/fixtures/**"]}));
    m.insert("labelMajorUpdates", json!({"matchUpdateTypes": ["major"], "labels": ["type: major"]}));
    m.insert("labelMinorUpdates", json!({"matchUpdateTypes": ["minor"], "labels": ["type: minor"]}));
    m.insert("labelPatchUpdates", json!({"matchUpdateTypes": ["patch"], "labels": ["type: patch"]}));
    m.insert("labelPinDigests", json!({"matchUpdateTypes": ["pinDigest"], "labels": ["type: pin"]}));
    m.insert("labelDigestUpdates", json!({"matchUpdateTypes": ["digest"], "labels": ["type: digest"]}));
    m.insert("maintainLockFilesDisabled", json!({"lockFileMaintenance": {"enabled": false}}));
    m.insert("maintainLockFilesWeekly", json!({"lockFileMaintenance": {"enabled": true, "schedule": ["before 5am on monday"]}}));
    m.insert("noSchedule", json!({"schedule": []}));
    m.insert("pinDigests", json!({"pinDigests": true}));
    m.insert("pinDependencies", json!({"matchUpdateTypes": ["pin"], "recreateWhen": "always"}));
    m.insert("pinOnlyDevDependencies", json!({"matchDepTypes": ["devDependencies"], "rangeStrategy": "pin", "matchUpdateTypes": ["pin"]}));
    m.insert("pinSkipCi", json!({"matchUpdateTypes": ["pin"], "semanticCommitScope": "", "semanticCommitType": "chore"}));
    m.insert("pinVersions", json!({"rangeStrategy": "pin"}));
    m.insert("prConcurrentLimit10", json!({"prConcurrentLimit": 10}));
    m.insert("prConcurrentLimit20", json!({"prConcurrentLimit": 20}));
    m.insert("prHourlyLimit1", json!({"prHourlyLimit": 1}));
    m.insert("prHourlyLimit2", json!({"prHourlyLimit": 2}));
    m.insert("prHourlyLimit4", json!({"prHourlyLimit": 4}));
    m.insert("prImmediately", json!({"prCreation": "immediate"}));
    m.insert("prNotPending", json!({"prCreation": "not-pending"}));
    m.insert("semanticCommits", json!({"semanticCommits": "enabled"}));
    m.insert("semanticCommitsDisabled", json!({"semanticCommits": "disabled"}));
    m.insert("semanticCommitScope(deprecated)", json!({}));
    m.insert("semanticCommitTypeAll(chore)", json!({"semanticCommitType": "chore"}));
    m.insert("semanticPrefixChore", json!({"semanticCommitType": "chore", "semanticCommitScope": ""}));
    m.insert("semanticPrefixFix", json!({"semanticCommitType": "fix", "semanticCommitScope": ""}));
    m.insert("separateMajorReleases", json!({"separateMajorMinor": true}));
    m.insert("separateMultipleMajorReleases", json!({"separateMultipleMajor": true}));
    m.insert("separatePatchReleases", json!({"separateMinorPatch": true}));
    m.insert("skipArtifactsUpdate", json!({"skipArtifactsUpdate": true}));
    m.insert("timezone", json!({}));
    m.insert("updateNotScheduled", json!({"updateNotScheduled": true}));
    m.insert("widenPeerDependencies", json!({"rangeStrategy": "widen", "matchDepTypes": ["peerDependencies"]}));
    m
}

fn config_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("js-app", json!({"extends": ["config:recommended", ":pinDependencies", "group:allNonMajor"]}));
    m.insert("js-lib", json!({"extends": ["config:recommended", "group:allNonMajor"]}));
    m.insert("recommended", json!({"extends": [":approveMajorUpdates", ":dependencyDashboard", ":semanticCommits", ":ignoreModulesAndTests", ":autodetectPinVersions", ":prImmediately", ":maintainLockFilesWeekly"]}));
    m.insert("semverAllMonthly", json!({"extends": ["config:recommended", ":automergeDigest", ":automergeBranch", "group:all", "schedule:monthly"]}));
    m.insert("semverAllWeekly", json!({"extends": ["config:recommended", ":automergeDigest", ":automergeBranch", "group:all", "schedule:weekly"]}));
    m
}

fn docker_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("disable", json!({"docker": {"enabled": false}}));
    m.insert("disableMajor", json!({"docker": {"major": {"enabled": false}}}));
    m.insert("pinDigests", json!({"docker": {"pinDigests": true}}));
    m
}

fn group_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("all", json!({"packageRules": [{"matchUpdateTypes": ["minor", "patch", "digest", "pin"], "groupName": "all non-major dependencies", "groupSlug": "all-minor-patch"}]}));
    m.insert("allNonMajor", json!({"packageRules": [{"matchUpdateTypes": ["minor", "patch"], "groupName": "all non-major dependencies", "groupSlug": "all-minor-patch"}]}));
    m.insert("monorepo", json!({}));
    m.insert("recommended", json!({"packageRules": [{"matchUpdateTypes": ["minor", "patch"], "groupName": "non-major dependencies", "groupSlug": "non-major"}]}));
    m
}

fn helpers_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("followTypespecVersioning", json!({"packageRules": [{"matchDatasources": ["npm"], "matchPackageNames": ["@types/*"], "versioning": "typescript"}]}));
    m.insert("followTypespecMajorVersioning", json!({"packageRules": [{"matchDatasources": ["npm"], "matchPackageNames": ["@types/*"], "versioning": "typescript", "matchUpdateTypes": ["major"]}, {"matchDatasources": ["npm"], "matchPackageNames": ["@types/*"], "versioning": "npm", "matchUpdateTypes": ["minor", "patch"]}]}));
    m
}

fn monorepo_presets() -> PresetMap {
    PresetMap::new()
}

fn packages_presets() -> PresetMap {
    PresetMap::new()
}

fn preview_presets() -> PresetMap {
    PresetMap::new()
}

fn schedule_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("daily", json!({"schedule": ["every day"]}));
    m.insert("earlyMondays", json!({"schedule": ["before 3am on Monday"]}));
    m.insert("weekdays", json!({"schedule": ["every weekday"]}));
    m.insert("weekends", json!({"schedule": ["every weekend"]}));
    m.insert("weekly", json!({"schedule": ["before 5am on Monday"]}));
    m.insert("monthly", json!({"schedule": ["before 5am on the first day of the month"]}));
    m.insert("quarterly", json!({"schedule": ["every 3 months on the first day of the month"]}));
    m.insert("yearly", json!({"schedule": ["every 12 months on the first day of the month"]}));
    m
}

fn security_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("minimumReleaseAgeNpm", json!({"packageRules": [{"matchDatasources": ["npm"], "minimumReleaseAge": "3 days"}]}));
    m
}

fn workarounds_presets() -> PresetMap {
    PresetMap::new()
}

fn replacements_presets() -> PresetMap {
    PresetMap::new()
}

fn merge_confidence_presets() -> PresetMap {
    let mut m = PresetMap::new();
    m.insert("all-badges", json!({"mergeConfidence": "high"}));
    m
}

fn custom_managers_presets() -> PresetMap {
    PresetMap::new()
}

fn abandonments_presets() -> PresetMap {
    PresetMap::new()
}

fn global_presets() -> PresetMap {
    PresetMap::new()
}

/// Get a built-in preset by group and name.
pub fn get_internal_preset(group: &str, name: &str) -> Option<Value> {
    INTERNAL_PRESETS.get(group).and_then(|m| m.get(name)).cloned()
}

/// List all available internal preset names.
pub fn list_internal_presets() -> Vec<String> {
    let mut result = Vec::new();
    for (group, presets) in INTERNAL_PRESETS.iter() {
        for name in presets.keys() {
            if *group == "default" {
                result.push(format!(":{name}"));
                result.push(format!("default:{name}"));
            } else {
                result.push(format!("{group}:{name}"));
            }
        }
    }
    result
}

/// Check if a preset string refers to an internal preset.
pub fn is_internal(preset: &str) -> bool {
    let without_params = preset.split('(').next().unwrap_or(preset);
    let all = list_internal_presets();
    all.iter().any(|p| p == without_params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_presets_contain_known_entries() {
        let preset = get_internal_preset("default", "pinVersions");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap()["rangeStrategy"], "pin");
    }

    #[test]
    fn config_recommended_exists() {
        let preset = get_internal_preset("config", "recommended");
        assert!(preset.is_some());
    }

    #[test]
    fn schedule_presets_contain_daily() {
        let preset = get_internal_preset("schedule", "daily");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap()["schedule"][0], "every day");
    }

    #[test]
    fn unknown_preset_returns_none() {
        assert!(get_internal_preset("nonexistent", "preset").is_none());
        assert!(get_internal_preset("default", "nonexistent").is_none());
    }

    #[test]
    fn list_internal_presets_is_nonempty() {
        let presets = list_internal_presets();
        assert!(!presets.is_empty());
        assert!(presets.contains(&":pinVersions".to_owned()));
        assert!(presets.contains(&"config:recommended".to_owned()));
    }

    #[test]
    fn is_internal_recognizes_known_presets() {
        assert!(is_internal(":pinVersions"));
        assert!(is_internal("config:recommended"));
        assert!(is_internal("schedule:daily"));
        assert!(!is_internal("github>owner/repo"));
    }

    #[test]
    fn is_internal_handles_parameterised_presets() {
        assert!(is_internal(":pinVersions(arg1)"));
    }
}

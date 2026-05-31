//! Common preset types and removed-preset mappings.
//!
//! Renovate reference: `lib/config/presets/common.ts`.

use std::collections::BTreeMap;

use serde_json::Value;

/// Presets that have been removed or renamed.
///
/// Mirrors `removedPresets` in `lib/config/presets/common.ts`.
/// A `None` value means the preset was removed (returns empty config).
/// A `Some(value)` means the preset was renamed to `value`.
pub fn removed_presets() -> &'static BTreeMap<&'static str, Option<&'static str>> {
    use std::sync::LazyLock;
    static PRESETS: LazyLock<BTreeMap<&str, Option<&str>>> = LazyLock::new(|| {
        let mut m = BTreeMap::new();
        m.insert(":autodetectPinVersions", None);
        m.insert(":autodetectRangeStrategy", None);
        m.insert(":automergeBranchMergeCommit", Some(":automergeBranch"));
        m.insert(":automergeBranchPush", Some(":automergeBranch"));
        m.insert(":base", Some("config:recommended"));
        m.insert(":app", Some("config:js-app"));
        m.insert(":disableLockFiles", Some(":skipArtifactsUpdate"));
        m.insert(":enableGradleLite", None);
        m.insert(":js-app", Some("config:js-app"));
        m.insert(":library", Some("config:js-lib"));
        m.insert(":masterIssue", Some(":dependencyDashboard"));
        m.insert(":masterIssueApproval", Some(":dependencyDashboardApproval"));
        m.insert(":switchToGradleLite", None);
        m.insert(":unpublishSafe", Some("npm:unpublishSafe"));
        m.insert("compatibility:additionalBranchPrefix", None);
        m.insert("config:application", Some("config:js-app"));
        m.insert("config:base", Some("config:recommended"));
        m.insert("config:base-js", Some("config:recommended"));
        m.insert("config:library", Some("config:js-lib"));
        m.insert("default:automergeBranchMergeCommit", Some(":automergeBranch"));
        m.insert("default:automergeBranchPush", Some(":automergeBranch"));
        m.insert("default:base", Some("config:recommended"));
        m.insert("default:app", Some("config:js-app"));
        m.insert("default:js-app", Some("config:js-app"));
        m.insert("default:library", Some("config:js-lib"));
        m.insert("default:onlyNpm", None);
        m.insert("default:unpublishSafe", Some("npm:unpublishSafe"));
        m.insert("helpers:oddIsUnstable", None);
        m.insert("helpers:oddIsUnstablePackages", None);
        m.insert("group:jsTestMonMajor", Some("group:jsTestNonMajor"));
        m.insert("group:kubernetes", Some("group:kubernetesMonorepo"));
        m.insert(
            "github>whitesource/merge-confidence:beta",
            Some("mergeConfidence:all-badges"),
        );
        m.insert("npm:unpublishSafe", Some("security:minimumReleaseAgeNpm"));
        m.insert(
            "replacements:messageFormat-{{package}}-to-@messageformat/{{package}}",
            Some("replacements:messageFormat-to-scoped"),
        );
        m.insert(
            "replacements:ojdbc10-to-ojdbc11",
            Some("replacements:ojdbc-to-ojdbc11"),
        );
        m.insert(
            "regexManagers:azurePipelinesVersions",
            Some("customManagers:azurePipelinesVersions"),
        );
        m.insert(
            "regexManagers:biomeVersions",
            Some("customManagers:biomeVersions"),
        );
        m.insert(
            "regexManagers:bitbucketPipelinesVersions",
            Some("customManagers:bitbucketPipelinesVersions"),
        );
        m.insert(
            "regexManagers:dockerfileVersions",
            Some("customManagers:dockerfileVersions"),
        );
        m.insert(
            "regexManagers:githubActionsVersions",
            Some("customManagers:githubActionsVersions"),
        );
        m.insert(
            "regexManagers:gitlabPipelineVersions",
            Some("customManagers:gitlabPipelineVersions"),
        );
        m.insert(
            "regexManagers:helmChartYamlAppVersions",
            Some("customManagers:helmChartYamlAppVersions"),
        );
        m.insert(
            "regexManagers:mavenPropertyVersions",
            Some("customManagers:mavenPropertyVersions"),
        );
        m.insert(
            "regexManagers:tfvarsVersions",
            Some("customManagers:tfvarsVersions"),
        );
        m.insert(
            "regexManagers:tsconfigNodeVersions",
            Some("customManagers:tsconfigNodeVersions"),
        );
        m.insert("workarounds:reduceRepologyServerLoad", None);
        m
    });
    &PRESETS
}

/// Renamed monorepo preset entries (mirrors `renamedMonorepos` in common.ts).
const RENAMED_MONOREPOS: &[(&str, &str)] = &[
    ("arcus event-grid", "arcus.event-grid"),
    ("arcus security", "arcus.security"),
    ("arcus messaging", "arcus.messaging"),
    ("arcus observability", "arcus.observability"),
    ("arcus webapi", "arcus.webapi"),
    ("arcus background-jobs", "arcus.background-jobs"),
    ("aspnet AspNetWebStack", "aspnet aspnetwebstack"),
    ("aspnet Extensions", "aspnet extensions"),
    ("k8s-io", "kubernetes"),
    ("System.IO.Abstractions", "system.io.abstractions"),
    ("angular1", "angularjs"),
    ("angularcli", "angular-cli"),
    ("Fontsource", "fontsource"),
    ("hamcrest", "javahamcrest"),
    ("Hangfire", "hangfire"),
    ("HotChocolate", "hotchocolate"),
    ("infrastructure", "infrastructure-ui"),
    ("junit5", "junit-framework"),
    ("lingui", "linguijs"),
    ("MassTransit", "masstransit"),
    ("material", "material-components-web"),
    ("mui", "material-ui"),
    ("openfeign", "feign"),
    ("opentelemetry", "opentelemetry-js"),
    ("OpenTelemetryDotnet", "opentelemetry-dotnet"),
    ("picasso", "picassojs"),
    ("reactrouter", "react-router"),
    ("sentry", "sentry-javascript"),
    ("Steeltoe", "steeltoe"),
    ("stryker", "stryker-js"),
    ("Swashbuckle", "swashbuckle-aspnetcore"),
    ("nrwl", "nx"),
];

/// Returns the full removed-presets map including renamed monorepo entries.
pub fn all_removed_presets() -> BTreeMap<String, Option<String>> {
    let mut map = BTreeMap::new();
    for (&k, &v) in removed_presets() {
        map.insert(k.to_owned(), v.map(str::to_owned));
    }
    for &(from, to) in RENAMED_MONOREPOS {
        map.insert(format!("monorepo:{from}"), Some(format!("monorepo:{to}")));
        map.insert(
            format!("group:{from}Monorepo"),
            Some(format!("group:{to}Monorepo")),
        );
    }
    map
}

/// A Preset is a config value (JSON object) that can be applied on top of
/// existing config. Mirrors `Preset` type from `lib/config/presets/types.ts`.
pub type Preset = serde_json::Map<String, Value>;

/// Merge a preset into a base config value.
///
/// Applies the preset on top of the base config using deep-merge semantics.
/// `packageRules` arrays are appended rather than replaced.
pub fn merge_preset(base: &Value, preset: &Value) -> Value {
    let result = base.clone();
    crate::config::merge_child_config(&result, Some(preset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn removed_presets_contains_known_entries() {
        let presets = removed_presets();
        assert_eq!(presets.get(":autodetectPinVersions"), Some(&None));
        assert_eq!(
            presets.get(":automergeBranchMergeCommit"),
            Some(&Some(":automergeBranch"))
        );
        assert_eq!(presets.get(":base"), Some(&Some("config:recommended")));
    }

    #[test]
    fn all_removed_presets_includes_monorepo_renames() {
        let presets = all_removed_presets();
        assert_eq!(
            presets.get("monorepo:angular1"),
            Some(&Some("monorepo:angularjs".to_owned()))
        );
        assert_eq!(
            presets.get("group:angular1Monorepo"),
            Some(&Some("group:angularjsMonorepo".to_owned()))
        );
    }

    #[test]
    fn merge_preset_appends_package_rules() {
        let base = json!({"packageRules": [{"a": 1}]});
        let preset = json!({"packageRules": [{"b": 2}]});
        let result = merge_preset(&base, &preset);
        assert_eq!(
            result["packageRules"],
            json!([{"a": 1}, {"b": 2}])
        );
    }
}

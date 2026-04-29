//! Monorepo grouping presets (`monorepo:*`, `group:*Monorepo`, `group:monorepos`).
//!
//! Resolves monorepo package grouping by reading the embedded monorepo.json
//! data file and generating `PackageRule` instances with appropriate
//! `matchPackageNames`, `matchSourceUrls`, and `groupName` fields.
//!
//! Renovate reference:
//! - `lib/data/monorepo.json` — source data (crowd-sourced)
//! - `lib/config/presets/internal/monorepos.preset.ts` — preset generation
//! - `lib/config/presets/internal/group.preset.ts` — group:monorepos expansion

use serde::Deserialize;

use crate::package_rule::PackageRule;
use crate::versioning::semver_generic::UpdateType;

static MONOREPO_JSON: &str = include_str!("../data/monorepo.json");

// ── Raw JSON types ──────────────────────────────────────────────────────────

/// A monorepo entry — either a single URL/pattern or a list of them.
#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum StringOrVec {
    One(String),
    Many(Vec<String>),
}

impl StringOrVec {
    fn to_vec(&self) -> Vec<String> {
        match self {
            Self::One(s) => vec![s.clone()],
            Self::Many(v) => v.clone(),
        }
    }
}

#[derive(Deserialize)]
struct MonorepoData {
    #[serde(rename = "repoGroups", default)]
    repo_groups: std::collections::HashMap<String, StringOrVec>,
    #[serde(rename = "orgGroups", default)]
    org_groups: std::collections::HashMap<String, StringOrVec>,
    #[serde(rename = "patternGroups", default)]
    pattern_groups: std::collections::HashMap<String, StringOrVec>,
}

fn load_data() -> Option<MonorepoData> {
    serde_json::from_str(MONOREPO_JSON).ok()
}

/// Non-pin update types (mirrors `nonPinUpdateTypes` in group.preset.ts).
/// Used by monorepo group rules to exclude pin updates.
fn non_pin_update_types() -> Vec<UpdateType> {
    // nonPinUpdateTypes = ['digest', 'patch', 'minor', 'major'] in Renovate.
    vec![
        UpdateType::Digest,
        UpdateType::Major,
        UpdateType::Minor,
        UpdateType::Patch,
    ]
}

/// Return the names of all `group:${name}Monorepo` presets that are part of
/// `group:monorepos`. Used by tests only — prefer `all_monorepo_rules()` in
/// production paths to avoid parsing the JSON multiple times.
pub fn all_monorepo_group_names() -> Vec<String> {
    let Some(data) = load_data() else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for key in data.pattern_groups.keys() {
        names.push(format!("group:{key}Monorepo"));
    }
    for key in data.org_groups.keys() {
        names.push(format!("group:{key}Monorepo"));
    }
    for key in data.repo_groups.keys() {
        names.push(format!("group:{key}Monorepo"));
    }
    names
}

/// Generate all monorepo `PackageRule` instances in a single JSON parse.
///
/// This is the efficient batch version used by `resolve_extends_group_presets`
/// when expanding `group:monorepos`. It parses the embedded JSON exactly once
/// rather than once per monorepo entry.
pub fn all_monorepo_rules() -> Vec<PackageRule> {
    let Some(data) = load_data() else {
        return Vec::new();
    };
    let update_types = non_pin_update_types();
    let mut rules = Vec::with_capacity(
        data.pattern_groups.len() + data.org_groups.len() + data.repo_groups.len(),
    );

    for (key, patterns) in &data.pattern_groups {
        let match_package_names = patterns.to_vec();
        let has_name = !match_package_names.is_empty();
        rules.push(PackageRule {
            match_package_names,
            has_name_constraint: has_name,
            match_update_types: update_types.clone(),
            has_update_type_constraint: true,
            group_name: Some(format!("{key} monorepo")),
            ..Default::default()
        });
    }

    for (key, urls) in &data.org_groups {
        let match_source_urls = urls
            .to_vec()
            .into_iter()
            .map(|u| format!("{u}**"))
            .collect::<Vec<_>>();
        rules.push(PackageRule {
            match_source_urls,
            match_update_types: update_types.clone(),
            has_update_type_constraint: true,
            group_name: Some(format!("{key} monorepo")),
            ..Default::default()
        });
    }

    for (key, urls) in &data.repo_groups {
        let match_source_urls = urls.to_vec();
        rules.push(PackageRule {
            match_source_urls,
            match_update_types: update_types.clone(),
            has_update_type_constraint: true,
            group_name: Some(format!("{key} monorepo")),
            ..Default::default()
        });
    }

    rules
}

/// Return the `PackageRule` instances for a specific `group:${name}Monorepo`
/// or `monorepo:${name}` preset.
///
/// - Pattern groups: matched by `matchPackageNames` (regex patterns)
/// - Org groups: matched by `matchSourceUrls` with `**` suffix
/// - Repo groups: matched by `matchSourceUrls` (exact or list)
///
/// Returns an empty vec when the monorepo name is unknown.
/// Prefer `all_monorepo_rules()` when expanding all monorepos at once.
pub fn rules_for_monorepo(name: &str) -> Vec<PackageRule> {
    let Some(data) = load_data() else {
        return Vec::new();
    };

    // Normalize: strip "Monorepo" suffix if present (group:angularMonorepo → angular)
    let key = name.strip_suffix("Monorepo").unwrap_or(name);

    let update_types = non_pin_update_types();
    let group_name = format!("{key} monorepo");

    // Check patternGroups first (matchPackageNames).
    if let Some(patterns) = data.pattern_groups.get(key) {
        let match_package_names = patterns.to_vec();
        let has_name = !match_package_names.is_empty();
        return vec![PackageRule {
            match_package_names,
            has_name_constraint: has_name,
            match_update_types: update_types,
            has_update_type_constraint: true,
            group_name: Some(group_name),
            ..Default::default()
        }];
    }

    // Check orgGroups (matchSourceUrls with ** suffix).
    if let Some(urls) = data.org_groups.get(key) {
        let match_source_urls = urls
            .to_vec()
            .into_iter()
            .map(|u| format!("{u}**"))
            .collect::<Vec<_>>();
        return vec![PackageRule {
            match_source_urls,
            match_update_types: update_types,
            has_update_type_constraint: true,
            group_name: Some(group_name),
            ..Default::default()
        }];
    }

    // Check repoGroups (matchSourceUrls, exact or list).
    if let Some(urls) = data.repo_groups.get(key) {
        let match_source_urls = urls.to_vec();
        return vec![PackageRule {
            match_source_urls,
            match_update_types: update_types,
            has_update_type_constraint: true,
            group_name: Some(group_name),
            ..Default::default()
        }];
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_monorepo_group_names_returns_non_empty() {
        let names = all_monorepo_group_names();
        assert!(!names.is_empty(), "must have monorepo group names");
        assert!(
            names.iter().any(|n| n.starts_with("group:")),
            "must produce group:* names"
        );
    }

    #[test]
    fn pattern_group_angularmaterial_resolved() {
        let rules = rules_for_monorepo("angularmaterial");
        assert!(!rules.is_empty(), "angularmaterial must resolve to rules");
        let rule = &rules[0];
        assert!(
            rule.group_name.as_deref() == Some("angularmaterial monorepo"),
            "must have correct group name"
        );
        assert!(
            rule.match_package_names
                .iter()
                .any(|p| p.contains("angular")),
            "must have angular package patterns"
        );
    }

    #[test]
    fn org_group_lodash_resolved_with_source_url() {
        let rules = rules_for_monorepo("lodash");
        assert!(!rules.is_empty(), "lodash org group must resolve to rules");
        let rule = &rules[0];
        assert!(
            rule.match_source_urls.iter().any(|u| u.contains("lodash")),
            "must have lodash source URL"
        );
    }

    #[test]
    fn group_name_monorepo_suffix_strips_correctly() {
        // group:angularmaterialMonorepo → key "angularmaterial"
        let rules = rules_for_monorepo("angularmaterialMonorepo");
        assert!(!rules.is_empty(), "angularmaterialMonorepo must resolve");
        assert_eq!(
            rules[0].group_name.as_deref(),
            Some("angularmaterial monorepo")
        );
    }

    #[test]
    fn unknown_monorepo_returns_empty() {
        let rules = rules_for_monorepo("nonexistent-repo");
        assert!(rules.is_empty(), "unknown monorepo must return empty vec");
    }

    #[test]
    fn monorepo_count_is_reasonable() {
        let names = all_monorepo_group_names();
        // Renovate ships 422 repoGroups + 9 orgGroups + 21 patternGroups = 452
        assert!(
            names.len() >= 400,
            "must have at least 400 monorepo groups, got {}",
            names.len()
        );
    }
}

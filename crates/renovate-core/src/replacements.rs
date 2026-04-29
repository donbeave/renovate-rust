//! Renovate crowd-sourced package replacement rules (`replacements:*` presets).
//!
//! Parses and serves the embedded `replacements.json` data from the Renovate
//! project, converting each entry into `PackageRule` instances that can be
//! prepended to the resolved rule set.
//!
//! Renovate reference:
//! - `lib/data/replacements.json` — source data (crowd-sourced)
//! - `lib/config/presets/internal/replacements.preset.ts` — preset generation

use serde::Deserialize;

use crate::package_rule::PackageRule;

/// Embedded JSON replacement data sourced from Renovate's replacements.json.
static REPLACEMENTS_JSON: &str = include_str!("../data/replacements.json");

// ── Raw JSON types ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RawRule {
    #[serde(rename = "matchPackageNames", default)]
    match_package_names: Vec<String>,
    #[serde(rename = "matchDatasources", default)]
    match_datasources: Vec<String>,
    #[serde(rename = "matchCurrentVersion")]
    match_current_version: Option<String>,
    #[serde(rename = "matchCurrentValue")]
    match_current_value: Option<String>,
    #[serde(rename = "replacementName")]
    replacement_name: Option<String>,
    #[serde(rename = "replacementVersion")]
    replacement_version: Option<String>,
}

#[derive(Deserialize)]
struct RawPreset {
    #[serde(rename = "packageRules", default)]
    package_rules: Vec<RawRule>,
}

/// Parse a single raw rule into a `PackageRule`.
fn raw_rule_to_package_rule(raw: RawRule) -> PackageRule {
    let has_name_constraint = !raw.match_package_names.is_empty();
    PackageRule {
        match_package_names: raw.match_package_names,
        has_name_constraint,
        match_datasources: raw.match_datasources,
        match_current_version: raw.match_current_version,
        match_current_value: raw.match_current_value,
        replacement_name: raw.replacement_name,
        replacement_version: raw.replacement_version,
        ..Default::default()
    }
}

/// Return the list of sub-preset names referenced by `replacements:all`.
///
/// Used by `expand_compound_presets` to expand `replacements:all`.
pub fn replacements_all_names() -> Vec<String> {
    let Ok(data) = serde_json::from_str::<serde_json::Value>(REPLACEMENTS_JSON) else {
        return Vec::new();
    };
    let Some(all) = data
        .get("all")
        .and_then(|v| v.get("extends"))
        .and_then(|e| e.as_array())
    else {
        return Vec::new();
    };
    all.iter()
        .filter_map(|v| v.as_str().map(|s| s.to_owned()))
        .collect()
}

/// Return the `PackageRule` instances for a specific `replacements:X` preset.
///
/// Returns an empty vec when the preset is unknown or has no packageRules.
pub fn rules_for_preset(preset_name: &str) -> Vec<PackageRule> {
    let Ok(data) = serde_json::from_str::<serde_json::Value>(REPLACEMENTS_JSON) else {
        return Vec::new();
    };
    let key = preset_name
        .strip_prefix("replacements:")
        .unwrap_or(preset_name);
    let Some(preset_value) = data.get(key) else {
        return Vec::new();
    };
    let Ok(preset) = serde_json::from_value::<RawPreset>(preset_value.clone()) else {
        return Vec::new();
    };
    preset
        .package_rules
        .into_iter()
        .map(raw_rule_to_package_rule)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacements_all_names_returns_non_empty_list() {
        let names = replacements_all_names();
        assert!(
            !names.is_empty(),
            "replacements:all must reference sub-presets"
        );
        assert!(
            names.iter().any(|n| n.starts_with("replacements:")),
            "replacements:all names must have replacements: prefix"
        );
    }

    #[test]
    fn babel_eslint_replacement_parsed() {
        let rules = rules_for_preset("babel-eslint-to-eslint-parser");
        assert!(
            !rules.is_empty(),
            "babel-eslint replacement must have rules"
        );
        let rule = &rules[0];
        assert!(
            rule.match_package_names
                .contains(&"babel-eslint".to_owned()),
            "must match babel-eslint"
        );
        assert_eq!(
            rule.replacement_name.as_deref(),
            Some("@babel/eslint-parser"),
            "must replace with @babel/eslint-parser"
        );
    }

    #[test]
    fn unknown_replacement_returns_empty() {
        let rules = rules_for_preset("nonexistent-replacement");
        assert!(rules.is_empty(), "unknown preset must return empty vec");
    }

    #[test]
    fn replacements_all_count_is_reasonable() {
        let names = replacements_all_names();
        // Renovate ships 60 individual replacement presets; allow ±10 for updates.
        assert!(
            names.len() >= 50,
            "replacements:all must reference at least 50 presets, got {}",
            names.len()
        );
    }
}

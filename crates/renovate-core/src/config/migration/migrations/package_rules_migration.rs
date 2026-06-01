use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

fn rename_keys(package_rule: &Map<String, Value>) -> Map<String, Value> {
    let mut new_rule = Map::new();
    for (key, val) in package_rule {
        let new_key = match key.as_str() {
            "matchFiles" | "matchPaths" | "paths" => "matchFileNames",
            "languages" | "matchLanguages" => "matchCategories",
            "baseBranchList" => "matchBaseBranches",
            "managers" => "matchManagers",
            "datasources" => "matchDatasources",
            "depTypeList" => "matchDepTypes",
            "packageNames" => "matchPackageNames",
            "packagePatterns" => "matchPackagePatterns",
            "sourceUrlPrefixes" => "matchSourceUrlPrefixes",
            "updateTypes" => "matchUpdateTypes",
            _ => key,
        };
        new_rule.insert(new_key.into(), val.clone());
    }
    new_rule
}

fn merge_matchers(package_rule: &Map<String, Value>) -> Map<String, Value> {
    let mut new_rule = package_rule.clone();

    for (key, val) in package_rule {
        let patterns: Vec<String> = if let Some(s) = val.as_str() {
            vec![s.into()]
        } else if let Some(arr) = val.as_array() {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.into())
                .collect()
        } else {
            continue;
        };

        match key.as_str() {
            "matchDepPrefixes" => {
                let target = new_rule
                    .entry("matchDepNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("{p}{{/,}}**")));
                    }
                }
                new_rule.remove("matchDepPrefixes");
            }
            "matchDepPatterns" => {
                let target = new_rule
                    .entry("matchDepNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("/{p}/")));
                    }
                }
                new_rule.remove("matchDepPatterns");
            }
            "excludeDepNames" => {
                let target = new_rule
                    .entry("matchDepNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("!{p}")));
                    }
                }
                new_rule.remove("excludeDepNames");
            }
            "excludeDepPrefixes" => {
                let target = new_rule
                    .entry("matchDepNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("!{p}{{/,}}**")));
                    }
                }
                new_rule.remove("excludeDepPrefixes");
            }
            "excludeDepPatterns" => {
                let target = new_rule
                    .entry("matchDepNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("!/{p}/")));
                    }
                }
                new_rule.remove("excludeDepPatterns");
            }
            "matchPackagePrefixes" => {
                let target = new_rule
                    .entry("matchPackageNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("{p}{{/,}}**")));
                    }
                }
                new_rule.remove("matchPackagePrefixes");
            }
            "matchPackagePatterns" => {
                let target = new_rule
                    .entry("matchPackageNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        if p == "*" {
                            arr.push(Value::String("*".into()));
                        } else {
                            arr.push(Value::String(format!("/{p}/")));
                        }
                    }
                }
                new_rule.remove("matchPackagePatterns");
            }
            "excludePackageNames" => {
                let target = new_rule
                    .entry("matchPackageNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("!{p}")));
                    }
                }
                new_rule.remove("excludePackageNames");
            }
            "excludePackagePrefixes" => {
                let target = new_rule
                    .entry("matchPackageNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("!{p}{{/,}}**")));
                    }
                }
                new_rule.remove("excludePackagePrefixes");
            }
            "excludePackagePatterns" => {
                let target = new_rule
                    .entry("matchPackageNames")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("!/{p}/")));
                    }
                }
                new_rule.remove("excludePackagePatterns");
            }
            "matchSourceUrlPrefixes" => {
                let target = new_rule
                    .entry("matchSourceUrls")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("{p}{{/,}}**")));
                    }
                }
                new_rule.remove("matchSourceUrlPrefixes");
            }
            "excludeRepositories" => {
                let target = new_rule
                    .entry("matchRepositories")
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(arr) = target {
                    for p in &patterns {
                        arr.push(Value::String(format!("!{p}")));
                    }
                }
                new_rule.remove("excludeRepositories");
            }
            _ => {}
        }
    }

    new_rule
}

#[derive(Clone, Debug)]
pub struct PackageRulesMigration;

impl Default for PackageRulesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageRulesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PackageRulesMigration {
    fn property_name(&self) -> &str {
        "packageRules"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let Value::Array(arr) = value else {
            return;
        };
        if arr.is_empty() {
            return;
        }

        let new_rules: Vec<Value> = arr
            .iter()
            .map(|rule| {
                let Value::Object(map) = rule else {
                    return rule.clone();
                };
                let renamed = rename_keys(map);
                let merged = merge_matchers(&renamed);
                Value::Object(merged)
            })
            .collect();

        migrated_config.insert("packageRules".into(), Value::Array(new_rules));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::PackageRulesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PackageRulesMigration::new();
        assert_eq!(m.property_name(), "packageRules");
    }

    #[test]
    fn renames_keys() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([{ "paths": [], "managers": [], "updateTypes": [] }]),
            &Map::new(),
            &mut migrated,
        );
        let rule = migrated["packageRules"].as_array().unwrap()[0]
            .as_object()
            .unwrap();
        assert!(rule.contains_key("matchFileNames"));
        assert!(rule.contains_key("matchManagers"));
        assert!(rule.contains_key("matchUpdateTypes"));
        assert!(!rule.contains_key("paths"));
        assert!(!rule.contains_key("managers"));
        assert!(!rule.contains_key("updateTypes"));
    }

    #[test]
    fn migrates_languages_to_categories() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([
                { "matchLanguages": ["docker", "js"], "addLabels": ["docker"] },
                { "languages": ["java"], "addLabels": ["java"] }
            ]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                { "matchCategories": ["docker", "js"], "addLabels": ["docker"] },
                { "matchCategories": ["java"], "addLabels": ["java"] }
            ])
        );
    }

    #[test]
    fn migrates_match_package_patterns() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([
                { "matchPackagePatterns": ["*"], "automerge": true },
                { "matchPackagePatterns": ["foo", "bar"], "automerge": true }
            ]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                { "automerge": true, "matchPackageNames": ["*"] },
                { "automerge": true, "matchPackageNames": ["/foo/", "/bar/"] }
            ])
        );
    }

    #[test]
    fn migrates_exclude_package_names() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([
                { "excludePackageNames": ["foo", "bar"], "automerge": true },
                { "matchPackageNames": ["baz"], "excludePackageNames": ["foo", "bar"], "automerge": true }
            ]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                { "automerge": true, "matchPackageNames": ["!foo", "!bar"] },
                { "automerge": true, "matchPackageNames": ["baz", "!foo", "!bar"] }
            ])
        );
    }

    #[test]
    fn migrates_string_values() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([
                {
                    "matchPackagePatterns": "pattern",
                    "matchPackagePrefixes": "prefix1",
                    "matchSourceUrlPrefixes": "prefix1",
                    "excludePackageNames": "excluded",
                    "excludePackagePatterns": "excludepattern",
                    "excludePackagePrefixes": "prefix1b",
                    "matchDepPatterns": "pattern",
                    "matchDepPrefixes": "prefix1",
                    "excludeDepNames": "excluded",
                    "excludeDepPatterns": "excludepattern",
                    "excludeDepPrefixes": "prefix1b",
                    "automerge": true
                }
            ]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                {
                    "matchPackageNames": [
                        "/pattern/",
                        "prefix1{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchDepNames": [
                        "/pattern/",
                        "prefix1{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchSourceUrls": ["prefix1{/,}**"],
                    "automerge": true
                }
            ])
        );
    }

    #[test]
    fn migrates_all_at_once() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([
                {
                    "matchPackagePatterns": ["pattern"],
                    "matchPackagePrefixes": ["prefix1", "prefix2"],
                    "matchSourceUrlPrefixes": ["prefix1", "prefix2"],
                    "excludePackageNames": ["excluded"],
                    "excludePackagePatterns": ["excludepattern"],
                    "excludePackagePrefixes": ["prefix1b"],
                    "matchPackageNames": ["mpn1", "mpn2"],
                    "matchDepPatterns": ["pattern"],
                    "matchDepPrefixes": ["prefix1", "prefix2"],
                    "excludeDepNames": ["excluded"],
                    "excludeDepPatterns": ["excludepattern"],
                    "excludeDepPrefixes": ["prefix1b"],
                    "matchDepNames": ["mpn1", "mpn2"],
                    "automerge": true
                }
            ]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                {
                    "matchPackageNames": [
                        "mpn1",
                        "mpn2",
                        "/pattern/",
                        "prefix1{/,}**",
                        "prefix2{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchDepNames": [
                        "mpn1",
                        "mpn2",
                        "/pattern/",
                        "prefix1{/,}**",
                        "prefix2{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchSourceUrls": ["prefix1{/,}**", "prefix2{/,}**"],
                    "automerge": true
                }
            ])
        );
    }

    #[test]
    fn preserves_config_order() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([{
                "paths": [],
                "labels": ["linting"],
                "baseBranchList": [],
                "languages": [],
                "managers": [],
                "datasources": [],
                "depTypeList": [],
                "addLabels": [],
                "packageNames": [],
                "updateTypes": []
            }]),
            &Map::new(),
            &mut migrated,
        );
        let rule = migrated["packageRules"].as_array().unwrap()[0]
            .as_object()
            .unwrap();
        let keys: Vec<_> = rule.keys().collect();
        assert_eq!(
            keys,
            vec![
                "matchFileNames",
                "labels",
                "matchBaseBranches",
                "matchCategories",
                "matchManagers",
                "matchDatasources",
                "matchDepTypes",
                "addLabels",
                "matchPackageNames",
                "matchUpdateTypes",
            ]
        );
    }

    #[test]
    fn does_not_migrate_nested_package_rules() {
        let m = PackageRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageRules",
            &json!([{
                "matchPaths": [],
                "packgageRules": { "languages": ["javascript"] }
            }]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([{
                "matchFileNames": [],
                "packgageRules": { "languages": ["javascript"] }
            }])
        );
    }
}

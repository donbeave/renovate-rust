//! Preset resolution — resolves extends into actual config.
//!
//! Renovate reference: `lib/config/presets/index.ts`.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use super::common::{all_removed_presets, merge_preset};
use super::parse::parse_preset;
use super::util::PRESET_NOT_FOUND;

/// Result of resolving config presets.
#[derive(Debug, Clone)]
pub struct PresetResult {
    pub config: Value,
    pub merged_presets: Vec<String>,
    pub unmerged_presets: Vec<String>,
}

/// Replace template arguments in a preset value.
///
/// Mirrors `replaceArgs()` from `lib/config/presets/index.ts`.
pub fn replace_args(obj: &Value, arg_mapping: &BTreeMap<String, String>) -> Value {
    match obj {
        Value::String(s) => {
            let mut result = s.clone();
            for (arg, val) in arg_mapping {
                let pattern = format!("{{{{{arg}}}}}");
                result = result.replace(&pattern, val);
            }
            Value::String(result)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(|v| replace_args(v, arg_mapping)).collect()),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), replace_args(v, arg_mapping)))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// Resolve config presets by processing extends and merging results.
///
/// This is a synchronous, non-recursive variant that processes internal presets only.
/// Network-based presets (github, gitlab, npm, http, local, forgejo, gitea) require
/// async runtime and are handled through the async variant.
///
/// Mirrors `resolveConfigPresets()` from `lib/config/presets/index.ts`.
pub fn resolve_config_presets(
    input_config: &Value,
    ignore_presets: Option<&[String]>,
    existing_presets: &[String],
) -> PresetResult {
    let mut merged_set: BTreeSet<String> = BTreeSet::new();
    let unmerged_set: BTreeSet<String> = BTreeSet::new();

    let ignore = ignore_presets
        .map(|s| s.to_vec())
        .or_else(|| input_config.get("ignorePresets").and_then(Value::as_array).map(|arr| {
            arr.iter().filter_map(Value::as_str).map(str::to_owned).collect()
        }))
        .unwrap_or_default();

    let mut config = json!({});

    if let Some(extends) = input_config.get("extends").and_then(Value::as_array) {
        for preset_str in extends {
            let preset = match preset_str.as_str() {
                Some(s) => s.to_owned(),
                None => continue,
            };

            if !should_resolve_preset(&preset, existing_presets, &ignore) {
                continue;
            }

            match fetch_preset_sync(&preset) {
                Ok(preset_config) => {
                    merged_set.insert(preset.clone());
                    config = merge_preset(&config, &preset_config);
                }
                Err(_) => {
                    merged_set.insert(preset);
                }
            }
        }
    }

    config = merge_preset(&config, input_config);
    if let Some(obj) = config.as_object_mut() {
        obj.remove("extends");
        obj.remove("ignorePresets");
    }

    PresetResult {
        config,
        merged_presets: merged_set.into_iter().collect(),
        unmerged_presets: unmerged_set.into_iter().collect(),
    }
}

fn should_resolve_preset(preset: &str, existing: &[String], ignore: &[String]) -> bool {
    if existing.iter().any(|p| p == preset) {
        return false;
    }
    if ignore.iter().any(|p| p == preset) {
        return false;
    }
    true
}

/// Synchronous preset fetcher that handles internal presets and renamed/removed presets.
fn fetch_preset_sync(preset: &str) -> Result<Value, String> {
    let removed = all_removed_presets();
    if let Some(Some(new_preset)) = removed.get(preset) {
        return fetch_preset_sync(new_preset);
    }
    if removed.get(preset) == Some(&None) {
        return Ok(json!({}));
    }

    let parsed = parse_preset(preset).map_err(|e| e)?;

    if parsed.preset_source == "internal" {
        let preset_config = super::internal::get_internal_preset(&parsed.repo, &parsed.preset_name);
        let Some(preset_config) = preset_config else {
            return Err(PRESET_NOT_FOUND.to_owned());
        };
        let mut config = preset_config;

        if let Some(params) = parsed.params {
            let mut arg_mapping = BTreeMap::new();
            for (i, value) in params.iter().enumerate() {
                arg_mapping.insert(format!("arg{i}"), value.clone());
            }
            if let Some(raw) = parsed.raw_params {
                arg_mapping.insert("args".to_owned(), raw);
            }
            config = replace_args(&config, &arg_mapping);
        }

        let config = crate::config::massage::massage_config(&config);
        Ok(config)
    } else {
        Err(format!(
            "Network-based preset source '{}' requires async runtime",
            parsed.preset_source
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn replace_args_replaces_placeholders() {
        let mut mapping = BTreeMap::new();
        mapping.insert("arg0".to_owned(), "value0".to_owned());
        let result = replace_args(&json!("prefix-{{arg0}}-suffix"), &mapping);
        assert_eq!(result, json!("prefix-value0-suffix"));
    }

    #[test]
    fn replace_args_handles_arrays_and_objects() {
        let mut mapping = BTreeMap::new();
        mapping.insert("arg0".to_owned(), "replaced".to_owned());
        let input = json!({"key": ["{{arg0}}"]});
        let result = replace_args(&input, &mapping);
        assert_eq!(result, json!({"key": ["replaced"]}));
    }

    #[test]
    fn resolve_config_presets_removes_extends() {
        let input = json!({"extends": [], "foo": "bar"});
        let result = resolve_config_presets(&input, None, &[]);
        assert!(result.config.get("extends").is_none());
        assert_eq!(result.config["foo"], "bar");
    }

    #[test]
    fn resolve_config_presets_no_extends() {
        let input = json!({"enabled": true});
        let result = resolve_config_presets(&input, None, &[]);
        assert_eq!(result.config["enabled"], true);
    }

    #[test]
    fn resolve_skips_ignored_presets() {
        let input = json!({"extends": [":pinVersions"], "ignorePresets": [":pinVersions"]});
        let result = resolve_config_presets(&input, None, &[]);
        assert!(!result.merged_presets.contains(&":pinVersions".to_owned()));
    }
}

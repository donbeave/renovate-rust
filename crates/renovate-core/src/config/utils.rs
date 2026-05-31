//! Config utility functions.
//!
//! Renovate reference: `lib/config/utils.ts`.

use serde_json::Value;

use super::merge_child_config;

/// Deep merge two config values.
///
/// This is a thin wrapper around `merge_child_config` for external use.
/// Mirrors `mergeChildConfig()` from `lib/config/utils.ts`.
pub fn merge_configs(parent: &Value, child: &Value) -> Value {
    merge_child_config(parent, Some(child))
}

/// Get config scoped to a specific package file path.
///
/// Returns a config object that has been filtered for the given package file.
/// For now, returns the base config as-is (to be extended with path-based filtering).
pub fn get_config_for_package_file(config: &Value, _package_file: &str) -> Value {
    config.clone()
}

/// Extract the highest vulnerability severity from parent and child configs.
pub fn get_highest_vulnerability_severity(
    parent: &Value,
    child: &Value,
) -> Option<String> {
    let severity_order = ["LOW", "MEDIUM", "HIGH", "CRITICAL"];
    let parent_severity = parent
        .get("vulnerabilitySeverity")
        .and_then(Value::as_str)
        .unwrap_or("");
    let child_severity = child
        .get("vulnerabilitySeverity")
        .and_then(Value::as_str)
        .unwrap_or("");

    let parent_idx = severity_order.iter().position(|&s| s == parent_severity);
    let child_idx = severity_order.iter().position(|&s| s == child_severity);

    match (parent_idx, child_idx) {
        (Some(pi), Some(ci)) => {
            if pi >= ci {
                Some(parent_severity.to_owned())
            } else {
                Some(child_severity.to_owned())
            }
        }
        (Some(_), None) => Some(parent_severity.to_owned()),
        (None, Some(_)) => Some(child_severity.to_owned()),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_configs_deep_merges() {
        let parent = json!({"constraints": {"node": ">=12"}, "labels": ["a"]});
        let child = json!({"constraints": {"npm": "^6"}, "labels": ["b"]});
        let result = merge_configs(&parent, &child);
        assert_eq!(result["constraints"]["node"], ">=12");
        assert_eq!(result["constraints"]["npm"], "^6");
    }

    #[test]
    fn get_config_for_package_file_returns_base() {
        let config = json!({"enabled": true});
        let result = get_config_for_package_file(&config, "package.json");
        assert_eq!(result["enabled"], true);
    }

    #[test]
    fn highest_severity_critical() {
        let parent = json!({"vulnerabilitySeverity": "HIGH"});
        let child = json!({"vulnerabilitySeverity": "CRITICAL"});
        assert_eq!(
            get_highest_vulnerability_severity(&parent, &child),
            Some("CRITICAL".to_owned())
        );
    }

    #[test]
    fn highest_severity_parent_wins() {
        let parent = json!({"vulnerabilitySeverity": "HIGH"});
        let child = json!({"vulnerabilitySeverity": "LOW"});
        assert_eq!(
            get_highest_vulnerability_severity(&parent, &child),
            Some("HIGH".to_owned())
        );
    }

    #[test]
    fn highest_severity_none() {
        let parent = json!({});
        let child = json!({});
        assert_eq!(get_highest_vulnerability_severity(&parent, &child), None);
    }
}

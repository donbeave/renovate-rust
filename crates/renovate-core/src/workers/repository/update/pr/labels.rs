//! PR label management.
//!
//! Mirrors `lib/workers/repository/update/pr/labels.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LabelConfig {
    pub labels: Vec<String>,
    pub add_labels: Vec<String>,
    pub dep_name: Option<String>,
    pub update_type: Option<String>,
    pub is_major: Option<bool>,
    pub is_minor: Option<bool>,
    pub is_patch: Option<bool>,
    pub is_pin: Option<bool>,
    pub is_digest: Option<bool>,
    pub is_vulnerability_alert: Option<bool>,
}

pub fn get_pr_labels(config: &LabelConfig) -> Vec<String> {
    let mut labels: Vec<String> = config.labels.iter().chain(config.add_labels.iter()).cloned().collect();

    if config.is_vulnerability_alert.unwrap_or(false) {
        let security = "security".to_owned();
        if !labels.contains(&security) {
            labels.push(security);
        }
    }

    match config.update_type.as_deref() {
        Some("major") if config.is_major.unwrap_or(false) => {
            let label = "type: major".to_owned();
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
        Some("minor") if config.is_minor.unwrap_or(false) => {
            let label = "type: minor".to_owned();
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
        Some("patch") if config.is_patch.unwrap_or(false) => {
            let label = "type: patch".to_owned();
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
        Some("pin") if config.is_pin.unwrap_or(false) => {
            let label = "type: pin".to_owned();
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
        Some("digest") if config.is_digest.unwrap_or(false) => {
            let label = "type: digest".to_owned();
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
        _ => {}
    }

    labels.sort();
    labels.dedup();
    labels
}

pub fn get_changed_labels(
    old_labels: &[String],
    new_labels: &[String],
) -> (Vec<String>, Vec<String>) {
    let to_add: Vec<String> = new_labels
        .iter()
        .filter(|l| !old_labels.contains(l))
        .cloned()
        .collect();

    let to_remove: Vec<String> = old_labels
        .iter()
        .filter(|l| !new_labels.contains(l))
        .cloned()
        .collect();

    (to_add, to_remove)
}

pub fn should_update_labels(
    pr_initial_labels: Option<&[String]>,
    pr_current_labels: &[String],
    configured_labels: &[String],
) -> bool {
    let Some(initial) = pr_initial_labels else {
        return false;
    };

    let mut sorted_configured = configured_labels.to_vec();
    sorted_configured.sort();
    let mut sorted_initial = initial.to_vec();
    sorted_initial.sort();

    if sorted_configured == sorted_initial {
        return false;
    }

    let mut sorted_current = pr_current_labels.to_vec();
    sorted_current.sort();
    let mut sorted_initial_check = initial.to_vec();
    sorted_initial_check.sort();

    if sorted_current != sorted_initial_check {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_config_default() {
        let c = LabelConfig::default();
        assert!(c.labels.is_empty());
        assert!(c.add_labels.is_empty());
        assert!(c.dep_name.is_none());
    }

    #[test]
    fn get_pr_labels_empty() {
        let config = LabelConfig::default();
        assert!(get_pr_labels(&config).is_empty());
    }

    #[test]
    fn get_pr_labels_base_labels() {
        let config = LabelConfig {
            labels: vec!["dependencies".into()],
            ..Default::default()
        };
        assert_eq!(get_pr_labels(&config), vec!["dependencies"]);
    }

    #[test]
    fn get_pr_labels_add_labels() {
        let config = LabelConfig {
            labels: vec!["dependencies".into()],
            add_labels: vec!["review".into()],
            ..Default::default()
        };
        assert_eq!(
            get_pr_labels(&config),
            vec!["dependencies", "review"]
        );
    }

    #[test]
    fn get_pr_labels_vulnerability() {
        let config = LabelConfig {
            is_vulnerability_alert: Some(true),
            ..Default::default()
        };
        let labels = get_pr_labels(&config);
        assert!(labels.contains(&"security".to_owned()));
    }

    #[test]
    fn get_pr_labels_major() {
        let config = LabelConfig {
            update_type: Some("major".into()),
            is_major: Some(true),
            ..Default::default()
        };
        let labels = get_pr_labels(&config);
        assert!(labels.contains(&"type: major".to_owned()));
    }

    #[test]
    fn get_pr_labels_minor() {
        let config = LabelConfig {
            update_type: Some("minor".into()),
            is_minor: Some(true),
            ..Default::default()
        };
        let labels = get_pr_labels(&config);
        assert!(labels.contains(&"type: minor".to_owned()));
    }

    #[test]
    fn get_pr_labels_patch() {
        let config = LabelConfig {
            update_type: Some("patch".into()),
            is_patch: Some(true),
            ..Default::default()
        };
        let labels = get_pr_labels(&config);
        assert!(labels.contains(&"type: patch".to_owned()));
    }

    #[test]
    fn get_pr_labels_dedup() {
        let config = LabelConfig {
            labels: vec!["dependencies".into(), "dependencies".into()],
            ..Default::default()
        };
        assert_eq!(get_pr_labels(&config), vec!["dependencies"]);
    }

    #[test]
    fn get_changed_labels_add_and_remove() {
        let old = vec!["a".into(), "b".into()];
        let new = vec!["b".into(), "c".into()];
        let (add, remove) = get_changed_labels(&old, &new);
        assert_eq!(add, vec!["c"]);
        assert_eq!(remove, vec!["a"]);
    }

    #[test]
    fn get_changed_labels_no_change() {
        let old = vec!["a".into(), "b".into()];
        let new = vec!["a".into(), "b".into()];
        let (add, remove) = get_changed_labels(&old, &new);
        assert!(add.is_empty());
        assert!(remove.is_empty());
    }

    #[test]
    fn should_update_labels_no_initial() {
        assert!(!should_update_labels(None, &[], &["a".into()]));
    }

    #[test]
    fn should_update_labels_same_as_initial() {
        let initial = vec!["a".into(), "b".into()];
        let current = vec!["a".into(), "b".into()];
        assert!(!should_update_labels(Some(&initial), &current, &initial));
    }

    #[test]
    fn should_update_labels_config_changed() {
        let initial = vec!["a".into()];
        let current = vec!["a".into()];
        let configured = vec!["b".into()];
        assert!(should_update_labels(
            Some(&initial),
            &current,
            &configured
        ));
    }

    #[test]
    fn should_update_labels_user_modified() {
        let initial = vec!["a".into()];
        let current = vec!["a".into(), "user-label".into()];
        let configured = vec!["b".into()];
        assert!(!should_update_labels(
            Some(&initial),
            &current,
            &configured
        ));
    }
}

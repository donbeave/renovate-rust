use regex::Regex;
use serde_json::Map;
use serde_json::Value;
use std::sync::LazyLock;

use crate::config::migration::Migration;

static PROPERTY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(azureAutoComplete|gitLabAutomerge)$").unwrap());

#[derive(Clone, Debug)]
pub struct AzureGitlabAutomergeMigration;

impl Default for AzureGitlabAutomergeMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl AzureGitlabAutomergeMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for AzureGitlabAutomergeMigration {
    fn property_name(&self) -> &str {
        "azureAutoComplete"
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn matches(&self, key: &str) -> bool {
        PROPERTY_RE.is_match(key)
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if let Some(b) = value.as_bool() {
            migrated_config.insert("platformAutomerge".into(), Value::Bool(b));
        }
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::AzureGitlabAutomergeMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = AzureGitlabAutomergeMigration::new();
        assert_eq!(m.property_name(), "azureAutoComplete");
    }

    #[test]
    fn matches_gitlab_automerge() {
        let m = AzureGitlabAutomergeMigration::new();
        assert!(m.matches("gitLabAutomerge"));
        assert!(m.matches("azureAutoComplete"));
        assert!(!m.matches("other"));
    }

    #[test]
    fn migrate_true() {
        let m = AzureGitlabAutomergeMigration::new();
        let mut migrated = Map::new();
        m.run("gitLabAutomerge", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["platformAutomerge"], json!(true));
    }

    #[test]
    fn migrate_false() {
        let m = AzureGitlabAutomergeMigration::new();
        let mut migrated = Map::new();
        m.run(
            "azureAutoComplete",
            &json!(false),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["platformAutomerge"], json!(false));
    }

    #[test]
    fn overrides_existing_platform_automerge() {
        let m = AzureGitlabAutomergeMigration::new();
        let mut migrated = Map::new();
        migrated.insert("platformAutomerge".into(), json!(false));
        m.run("gitLabAutomerge", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["platformAutomerge"], json!(true));
    }

    #[test]
    fn undefined_value_is_noop() {
        let m = AzureGitlabAutomergeMigration::new();
        let mut migrated = Map::new();
        m.run("gitLabAutomerge", &json!(null), &Map::new(), &mut migrated);
        assert!(migrated.get("platformAutomerge").is_none());
    }
}

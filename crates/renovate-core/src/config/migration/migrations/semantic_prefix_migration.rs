use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug, Default)]
pub struct SemanticPrefixMigration;

impl SemanticPrefixMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for SemanticPrefixMigration {
    fn property_name(&self) -> &str {
        "semanticPrefix"
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let Some(s) = value.as_str() else {
            return;
        };
        let Some((text, _)) = s.split_once(':') else {
            return;
        };
        let (type_, scope) = if let Some((t, rest)) = text.split_once('(') {
            let scope = rest.strip_suffix(')').unwrap_or(rest);
            (t, Some(scope))
        } else {
            (text, None)
        };

        if !migrated_config.contains_key("semanticCommitType") {
            migrated_config.insert("semanticCommitType".into(), Value::String(type_.into()));
        }
        if let Some(sc) = scope
            && !migrated_config.contains_key("semanticCommitScope")
        {
            migrated_config.insert("semanticCommitScope".into(), Value::String(sc.into()));
        }
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::Map;

    use super::SemanticPrefixMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = SemanticPrefixMigration::new();
        assert_eq!(m.property_name(), "semanticPrefix");
    }

    #[test]
    fn parses_type_and_scope() {
        let m = SemanticPrefixMigration::new();
        let mut migrated = Map::new();
        m.run("semanticPrefix", &json!("fix(deps):"), &Map::new(), &mut migrated);
        assert_eq!(migrated["semanticCommitType"], json!("fix"));
        assert_eq!(migrated["semanticCommitScope"], json!("deps"));
    }

    #[test]
    fn parses_type_without_scope() {
        let m = SemanticPrefixMigration::new();
        let mut migrated = Map::new();
        m.run("semanticPrefix", &json!("chore:"), &Map::new(), &mut migrated);
        assert_eq!(migrated["semanticCommitType"], json!("chore"));
        assert!(!migrated.contains_key("semanticCommitScope"));
    }

    #[test]
    fn does_not_overwrite_existing() {
        let m = SemanticPrefixMigration::new();
        let mut migrated = Map::new();
        migrated.insert("semanticCommitType".into(), json!("feat"));
        migrated.insert("semanticCommitScope".into(), json!("build"));
        m.run("semanticPrefix", &json!("fix(deps):"), &Map::new(), &mut migrated);
        assert_eq!(migrated["semanticCommitType"], json!("feat"));
        assert_eq!(migrated["semanticCommitScope"], json!("build"));
    }
}

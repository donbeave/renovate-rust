pub mod migrations;

use serde_json::Map;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use self::migrations::automerge_migration::AutomergeMigration;
use self::migrations::binary_source_migration::BinarySourceMigration;
use self::migrations::branch_name_migration::BranchNameMigration;
use self::migrations::rename_property_migration::RenamePropertyMigration;
use self::migrations::schedule_migration::ScheduleMigration;

pub trait Migration: Send + Sync {
    fn property_name(&self) -> &str;
    fn is_deprecated(&self) -> bool {
        false
    }
    fn run(
        &self,
        key: &str,
        value: &Value,
        original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    );
    fn box_clone(&self) -> Box<dyn Migration>;
}

pub struct MigrationService {
    removed_properties: BTreeSet<&'static str>,
    renamed_properties: BTreeMap<&'static str, &'static str>,
    custom_migrations: Vec<Box<dyn Migration>>,
}

impl std::fmt::Debug for MigrationService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MigrationService")
            .field("removed_properties", &self.removed_properties)
            .field("renamed_properties", &self.renamed_properties)
            .field("custom_migrations", &self.custom_migrations.len())
            .finish()
    }
}

impl Default for MigrationService {
    fn default() -> Self {
        Self::new()
    }
}

impl MigrationService {
    pub fn new() -> Self {
        Self {
            removed_properties: Self::default_removed_properties(),
            renamed_properties: Self::default_renamed_properties(),
            custom_migrations: Self::default_custom_migrations(),
        }
    }

    fn default_removed_properties() -> BTreeSet<&'static str> {
        [
            "allowCommandTemplating",
            "allowPostUpgradeCommandTemplating",
            "deepExtract",
            "gitFs",
            "groupBranchName",
            "groupCommitMessage",
            "groupPrBody",
            "groupPrTitle",
            "lazyGrouping",
            "maintainYarnLock",
            "raiseDeprecationWarnings",
            "statusCheckVerify",
            "supportPolicy",
            "transitiveRemediation",
            "yarnCacheFolder",
            "yarnMaintenanceBranchName",
            "yarnMaintenanceCommitMessage",
            "yarnMaintenancePrBody",
            "yarnMaintenancePrTitle",
        ]
        .into_iter()
        .collect()
    }

    fn default_renamed_properties() -> BTreeMap<&'static str, &'static str> {
        [
            ("adoptium-java", "java-version"),
            ("allowedPostUpgradeCommands", "allowedCommands"),
            ("azureAutoApprove", "autoApprove"),
            ("customChangelogUrl", "changelogUrl"),
            ("endpoints", "hostRules"),
            ("excludedPackageNames", "excludePackageNames"),
            ("exposeEnv", "exposeAllEnv"),
            ("keepalive", "keepAlive"),
            ("managerBranchPrefix", "additionalBranchPrefix"),
            ("multipleMajorPrs", "separateMultipleMajor"),
            ("separatePatchReleases", "separateMinorPatch"),
            ("versionScheme", "versioning"),
            ("lookupNameTemplate", "packageNameTemplate"),
            ("aliases", "registryAliases"),
            ("masterIssue", "dependencyDashboard"),
            ("masterIssueApproval", "dependencyDashboardApproval"),
            ("masterIssueAutoclose", "dependencyDashboardAutoclose"),
            ("masterIssueHeader", "dependencyDashboardHeader"),
            ("masterIssueFooter", "dependencyDashboardFooter"),
            ("masterIssueTitle", "dependencyDashboardTitle"),
            ("masterIssueLabels", "dependencyDashboardLabels"),
            ("regexManagers", "customManagers"),
            ("baseBranches", "baseBranchPatterns"),
        ]
        .into_iter()
        .collect()
    }

    fn default_custom_migrations() -> Vec<Box<dyn Migration>> {
        vec![
            Box::new(AutomergeMigration::new()),
            Box::new(BinarySourceMigration::new()),
            Box::new(BranchNameMigration::new()),
            Box::new(ScheduleMigration::new()),
        ]
    }

    pub fn run(&self, original_config: &Map<String, Value>) -> Map<String, Value> {
        let mut migrated_config = original_config.clone();
        let all_migrations = self.build_migrations();

        for (key, value) in original_config {
            if let Some(migration) = self.find_migration(&all_migrations, key) {
                migration.run(key, value, original_config, &mut migrated_config);
                if migration.is_deprecated() {
                    migrated_config.remove(key);
                }
            }
        }

        migrated_config
    }

    fn build_migrations(&self) -> Vec<Box<dyn Migration>> {
        let mut migrations: Vec<Box<dyn Migration>> = Vec::new();

        for &prop in &self.removed_properties {
            migrations.push(Box::new(RemovePropertyMigration::new(prop)));
        }

        for (&old_name, &new_name) in &self.renamed_properties {
            migrations.push(Box::new(RenamePropertyMigration::new(old_name, new_name)));
        }

        for m in &self.custom_migrations {
            migrations.push(m.box_clone());
        }

        migrations
    }

    fn find_migration<'a>(
        &self,
        migrations: &'a [Box<dyn Migration>],
        key: &str,
    ) -> Option<&'a dyn Migration> {
        migrations.iter().find(|m| m.property_name() == key).map(|b| b.as_ref())
    }

    pub fn is_migrated(
        original: &Map<String, Value>,
        migrated: &Map<String, Value>,
    ) -> bool {
        original != migrated
    }
}

struct RemovePropertyMigration {
    property: &'static str,
}

impl RemovePropertyMigration {
    fn new(property: &'static str) -> Self {
        Self { property }
    }
}

impl Clone for RemovePropertyMigration {
    fn clone(&self) -> Self {
        Self { property: self.property }
    }
}

impl Migration for RemovePropertyMigration {
    fn property_name(&self) -> &str {
        self.property
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn run(
        &self,
        _key: &str,
        _value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        migrated_config.remove(self.property);
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

impl Clone for MigrationService {
    fn clone(&self) -> Self {
        Self {
            removed_properties: self.removed_properties.clone(),
            renamed_properties: self.renamed_properties.clone(),
            custom_migrations: self
                .custom_migrations
                .iter()
                .map(|m| m.box_clone())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::Map;

    use super::MigrationService;

    fn map_from_value(value: serde_json::Value) -> Map<String, serde_json::Value> {
        match value {
            serde_json::Value::Object(map) => map,
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn run_returns_unchanged_config_when_no_migrations_apply() {
        let original = map_from_value(json!({"enabled": true}));
        let service = MigrationService::new();
        let migrated = service.run(&original);
        assert_eq!(migrated["enabled"], json!(true));
    }

    #[test]
    fn run_removes_removed_properties() {
        let original = map_from_value(json!({
            "enabled": true,
            "gitFs": "https"
        }));
        let service = MigrationService::new();
        let migrated = service.run(&original);
        assert!(migrated.get("gitFs").is_none());
        assert_eq!(migrated["enabled"], json!(true));
    }

    #[test]
    fn run_renamed_properties() {
        let original = map_from_value(json!({
            "versionScheme": "semver"
        }));
        let service = MigrationService::new();
        let migrated = service.run(&original);
        assert!(migrated.get("versionScheme").is_none());
        assert_eq!(migrated["versioning"], json!("semver"));
    }

    #[test]
    fn run_does_not_overwrite_existing_new_property_on_rename() {
        let original = map_from_value(json!({
            "versionScheme": "semver",
            "versioning": "npm"
        }));
        let service = MigrationService::new();
        let migrated = service.run(&original);
        assert!(migrated.get("versionScheme").is_none());
        assert_eq!(migrated["versioning"], json!("npm"));
    }

    #[test]
    fn is_migrated_returns_true_when_configs_differ() {
        let a = map_from_value(json!({"a": 1}));
        let b = map_from_value(json!({"a": 2}));
        assert!(MigrationService::is_migrated(&a, &b));
    }

    #[test]
    fn is_migrated_returns_false_when_configs_match() {
        let a = map_from_value(json!({"a": 1}));
        let b = map_from_value(json!({"a": 1}));
        assert!(!MigrationService::is_migrated(&a, &b));
    }
}

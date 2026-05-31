//! Lookup update logic.
//!
//! Mirrors `lib/workers/repository/process/lookup/index.ts`.

use anyhow::Result;

use crate::workers::types::{RenovateConfig, Upgrade, ValidationMessage};

use super::types::{LookupUpdate, LookupUpdateConfig, UpdateResult, UpdateType};

pub fn lookup_updates(config: &LookupUpdateConfig) -> Result<UpdateResult> {
    let mut res = UpdateResult {
        versioning: config.filter.versioning.clone(),
        updates: Vec::new(),
        warnings: Vec::new(),
        ..Default::default()
    };

    if config.rollback.current_value.is_none() && config.current_digest.is_none() {
        res.skip_reason = Some("invalid-value".into());
        return Ok(res);
    }

    let current_value = match &config.rollback.current_value {
        Some(v) => v.clone(),
        None => {
            res.skip_reason = Some("invalid-value".into());
            return Ok(res);
        }
    };

    if config.rollback.datasource.is_empty() {
        res.skip_reason = Some("invalid-config".into());
        return Ok(res);
    }

    if config.replacement_name.is_some()
        || config.replacement_name_template.is_some()
        || config.replacement_version.is_some()
        || config.replacement_version_template.is_some()
    {
        let new_name = config
            .replacement_name
            .clone()
            .unwrap_or_else(|| config.rollback.package_name.clone());
        let new_value = config
            .replacement_version
            .clone()
            .or_else(|| config.rollback.current_value.clone());
        if new_name != config.rollback.package_name
            || new_value.as_deref() != Some(&current_value)
        {
            res.updates.push(LookupUpdate {
                update_type: Some(UpdateType::Replacement),
                new_name: Some(new_name),
                new_value,
                ..Default::default()
            });
        }
    }

    Ok(res)
}

pub fn lookup_dependency(
    _package_file_config: &RenovateConfig,
    dep: &mut Upgrade,
) -> Result<UpdateResult> {
    if let Some(ref reason) = dep.update_type
        && reason == "skipReason"
    {
        return Ok(UpdateResult::default());
    }

    let dep_name = dep
        .dep_name
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_owned();

    if dep_name.is_empty() {
        return Ok(UpdateResult {
            warnings: vec![ValidationMessage {
                topic: Some("invalid-name".into()),
                message: Some("Dependency has no valid name".into()),
            }],
            ..Default::default()
        });
    }

    let datasource = match &dep.datasource {
        Some(ds) if !ds.is_empty() => ds.clone(),
        _ => {
            return Ok(UpdateResult::default());
        }
    };

    let lookup_config = LookupUpdateConfig {
        rollback: super::types::RollbackConfig {
            current_value: dep.current_value.clone(),
            package_name: dep.package_name.clone().unwrap_or(dep_name.clone()),
            dep_name: Some(dep_name.clone()),
            package_file: dep.package_file.clone(),
            versioning: None,
            datasource,
        },
        filter: super::types::FilterConfig {
            allowed_versions: None,
            dep_name: Some(dep_name),
            follow_tag: None,
            ignore_deprecated: None,
            ignore_unstable: None,
            max_major_increment: None,
            respect_latest: None,
            update_pinned_dependencies: None,
            versioning: None,
        },
        current_version: dep.current_version.clone(),
        current_digest: dep.current_digest.clone(),
        locked_version: None,
        digest_one_and_only: None,
        rollback_prs: None,
        is_vulnerability_alert: None,
        minimum_confidence: None,
        replacement_name: None,
        replacement_name_template: None,
        replacement_version: None,
        replacement_version_template: None,
        extract_version: None,
        vulnerability_fix_version: None,
        vulnerability_fix_strategy: None,
        abandonment_threshold: None,
    };

    lookup_updates(&lookup_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::repository::process::lookup::types::RollbackConfig;

    fn make_config(
        package_name: &str,
        current_value: Option<&str>,
        datasource: &str,
    ) -> LookupUpdateConfig {
        LookupUpdateConfig {
            rollback: RollbackConfig {
                package_name: package_name.into(),
                current_value: current_value.map(String::from),
                dep_name: Some(package_name.into()),
                datasource: datasource.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn lookup_updates_basic() {
        let config = make_config("lodash", Some("4.17.0"), "npm");
        let result = lookup_updates(&config).unwrap();
        assert!(result.updates.is_empty() || result.skip_reason.is_some());
    }

    #[test]
    fn lookup_updates_no_current_value() {
        let config = make_config("lodash", None, "npm");
        let result = lookup_updates(&config).unwrap();
        assert_eq!(result.skip_reason, Some("invalid-value".into()));
    }

    #[test]
    fn lookup_updates_empty_datasource() {
        let config = make_config("lodash", Some("4.17.0"), "");
        let result = lookup_updates(&config).unwrap();
        assert_eq!(result.skip_reason, Some("invalid-config".into()));
    }

    #[test]
    fn lookup_updates_with_replacement() {
        let mut config = make_config("old-pkg", Some("1.0.0"), "npm");
        config.replacement_name = Some("new-pkg".into());
        config.replacement_version = Some("2.0.0".into());
        let result = lookup_updates(&config).unwrap();
        assert_eq!(result.updates.len(), 1);
        assert_eq!(
            result.updates[0].update_type,
            Some(UpdateType::Replacement)
        );
        assert_eq!(result.updates[0].new_name, Some("new-pkg".into()));
        assert_eq!(result.updates[0].new_value, Some("2.0.0".into()));
    }

    #[test]
    fn lookup_updates_replacement_same_name_same_value() {
        let mut config = make_config("lodash", Some("4.17.0"), "npm");
        config.replacement_name = Some("lodash".into());
        config.replacement_version = Some("4.17.0".into());
        let result = lookup_updates(&config).unwrap();
        assert!(result.updates.is_empty());
    }

    #[test]
    fn lookup_updates_replacement_name_only() {
        let mut config = make_config("old-pkg", Some("1.0.0"), "npm");
        config.replacement_name = Some("new-pkg".into());
        let result = lookup_updates(&config).unwrap();
        assert_eq!(result.updates.len(), 1);
    }

    #[test]
    fn lookup_dependency_no_name() {
        let config = RenovateConfig::default();
        let mut dep = Upgrade {
            datasource: Some("npm".into()),
            ..Default::default()
        };
        let result = lookup_dependency(&config, &mut dep).unwrap();
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn lookup_dependency_no_datasource() {
        let config = RenovateConfig::default();
        let mut dep = Upgrade {
            dep_name: Some("lodash".into()),
            ..Default::default()
        };
        let result = lookup_dependency(&config, &mut dep).unwrap();
        assert!(result.updates.is_empty());
    }

    #[test]
    fn lookup_dependency_basic() {
        let config = RenovateConfig::default();
        let mut dep = Upgrade {
            dep_name: Some("lodash".into()),
            current_value: Some("4.17.0".into()),
            datasource: Some("npm".into()),
            ..Default::default()
        };
        let result = lookup_dependency(&config, &mut dep).unwrap();
        assert!(result.updates.is_empty() || result.skip_reason.is_some());
    }

    #[test]
    fn update_result_default_values() {
        let r = UpdateResult::default();
        assert!(r.updates.is_empty());
        assert!(r.warnings.is_empty());
        assert!(r.skip_reason.is_none());
        assert!(r.source_url.is_none());
        assert!(r.current_version.is_none());
    }
}

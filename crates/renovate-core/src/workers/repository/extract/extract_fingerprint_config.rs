//! Extract fingerprint config.
//!
//! Mirrors `lib/workers/repository/extract/extract-fingerprint-config.ts`.

use std::collections::HashMap;

use crate::workers::types::UpgradeFingerprintConfig;

pub fn get_extract_fingerprint_config(
    upgrade: &UpgradeFingerprintConfig,
) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &upgrade.auto_replace_string_template {
        parts.push(format!("auto_replace_string_template={v}"));
    }
    if let Some(v) = &upgrade.current_digest {
        parts.push(format!("current_digest={v}"));
    }
    if let Some(v) = &upgrade.current_value {
        parts.push(format!("current_value={v}"));
    }
    if let Some(v) = &upgrade.current_version {
        parts.push(format!("current_version={v}"));
    }
    if let Some(v) = &upgrade.datasource {
        parts.push(format!("datasource={v}"));
    }
    if let Some(v) = &upgrade.dep_name {
        parts.push(format!("dep_name={v}"));
    }
    if let Some(v) = &upgrade.lock_file {
        parts.push(format!("lock_file={v}"));
    }
    if let Some(v) = &upgrade.manager {
        parts.push(format!("manager={v}"));
    }
    if let Some(v) = &upgrade.new_name {
        parts.push(format!("new_name={v}"));
    }
    if let Some(v) = &upgrade.package_file {
        parts.push(format!("package_file={v}"));
    }
    parts.sort();
    parts.join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_extract_fingerprint_config_empty() {
        let config = UpgradeFingerprintConfig::default();
        let fp = get_extract_fingerprint_config(&config);
        assert!(fp.is_empty());
    }

    #[test]
    fn get_extract_fingerprint_config_with_fields() {
        let config = UpgradeFingerprintConfig {
            dep_name: Some("lodash".into()),
            current_value: Some("4.17.0".into()),
            datasource: Some("npm".into()),
            manager: Some("npm".into()),
            ..Default::default()
        };
        let fp = get_extract_fingerprint_config(&config);
        assert!(fp.contains("current_value=4.17.0"));
        assert!(fp.contains("datasource=npm"));
        assert!(fp.contains("dep_name=lodash"));
        assert!(fp.contains("manager=npm"));
    }

    #[test]
    fn get_extract_fingerprint_config_deterministic() {
        let config = UpgradeFingerprintConfig {
            dep_name: Some("pkg".into()),
            current_value: Some("1.0".into()),
            ..Default::default()
        };
        let fp1 = get_extract_fingerprint_config(&config);
        let fp2 = get_extract_fingerprint_config(&config);
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn get_extract_fingerprint_config_sorted() {
        let config = UpgradeFingerprintConfig {
            datasource: Some("npm".into()),
            dep_name: Some("lodash".into()),
            ..Default::default()
        };
        let fp = get_extract_fingerprint_config(&config);
        let dep_pos = fp.find("dep_name=").unwrap();
        let ds_pos = fp.find("datasource=").unwrap();
        assert!(dep_pos < ds_pos);
    }
}

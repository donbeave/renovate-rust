use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct JsonataExtractConfig {
    pub match_strings: Vec<String>,
    pub file_format: Option<String>,
    pub dep_name_template: Option<String>,
    pub current_value_template: Option<String>,
    pub datasource_template: Option<String>,
}

#[derive(Debug, Clone)]
pub struct JsonataDependency {
    pub dep_name: Option<String>,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub datasource: Option<String>,
    pub versioning: Option<String>,
    pub registry_url: Option<String>,
    pub dep_type: Option<String>,
}

pub fn extract_package_file(
    content: &str,
    package_file: &str,
    config: &JsonataExtractConfig,
) -> Option<Vec<JsonataDependency>> {
    let json = match config.file_format.as_deref() {
        Some("json") | None => serde_json::from_str(content).ok()?,
        Some("yaml") => {
            let yaml_val: serde_yaml::Value = serde_yaml::from_str(content).ok()?;
            serde_json::to_value(yaml_val).ok()?
        }
        Some("toml") => {
            let toml_val: toml::Value = toml::from_str(content).ok()?;
            serde_json::to_value(toml_val).ok()?
        }
        _ => return None,
    };

    let deps = handle_matching(&json, package_file, config);
    if deps.is_empty() {
        return None;
    }
    Some(deps)
}

fn handle_matching(
    json: &Value,
    _package_file: &str,
    config: &JsonataExtractConfig,
) -> Vec<JsonataDependency> {
    let mut results = Vec::new();

    if let Some(objs) = json.as_array() {
        for item in objs {
            if let Some(dep) = create_dependency_from_value(item) {
                results.push(dep);
            }
        }
    } else if let Some(obj) = json.as_object() {
        if let Some(dep) = create_dependency_from_value(&Value::Object(obj.clone())) {
            results.push(dep);
        }
    }

    if results.is_empty() {
        if let Some(dep_name) = json.get("depName").and_then(|v| v.as_str()) {
            let dep = JsonataDependency {
                dep_name: Some(dep_name.to_owned()),
                current_value: json.get("currentValue").and_then(|v| v.as_str()).map(|s| s.to_owned()),
                current_digest: json.get("currentDigest").and_then(|v| v.as_str()).map(|s| s.to_owned()),
                datasource: json.get("datasource").and_then(|v| v.as_str()).map(|s| s.to_owned()),
                versioning: json.get("versioning").and_then(|v| v.as_str()).map(|s| s.to_owned()),
                registry_url: json.get("registryUrl").and_then(|v| v.as_str()).map(|s| s.to_owned()),
                dep_type: json.get("depType").and_then(|v| v.as_str()).map(|s| s.to_owned()),
            };
            if dep.dep_name.is_some() && (dep.current_value.is_some() || dep.current_digest.is_some()) {
                results.push(dep);
            }
        }
    }

    let _ = config;
    results
}

fn create_dependency_from_value(val: &Value) -> Option<JsonataDependency> {
    let dep_name = val.get("depName").and_then(|v| v.as_str()).map(|s| s.to_owned());
    let current_value = val.get("currentValue").and_then(|v| v.as_str()).map(|s| s.to_owned());
    let current_digest = val.get("currentDigest").and_then(|v| v.as_str()).map(|s| s.to_owned());

    if dep_name.is_none() && current_value.is_none() && current_digest.is_none() {
        return None;
    }

    Some(JsonataDependency {
        dep_name,
        current_value,
        current_digest,
        datasource: val.get("datasource").and_then(|v| v.as_str()).map(|s| s.to_owned()),
        versioning: val.get("versioning").and_then(|v| v.as_str()).map(|s| s.to_owned()),
        registry_url: val.get("registryUrl").and_then(|v| v.as_str()).map(|s| s.to_owned()),
        dep_type: val.get("depType").and_then(|v| v.as_str()).map(|s| s.to_owned()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_from_json_array() {
        let content = r#"[{"depName":"nginx","currentValue":"1.23.0"},{"depName":"redis","currentValue":"7.0.0"}]"#;
        let config = JsonataExtractConfig {
            file_format: Some("json".to_owned()),
            ..Default::default()
        };
        let deps = extract_package_file(content, "test.json", &config).unwrap();
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name.as_deref(), Some("nginx"));
        assert_eq!(deps[1].dep_name.as_deref(), Some("redis"));
    }

    #[test]
    fn extract_from_json_object() {
        let content = r#"{"depName":"myapp","currentValue":"2.0.0","datasource":"npm"}"#;
        let config = JsonataExtractConfig {
            file_format: Some("json".to_owned()),
            ..Default::default()
        };
        let deps = extract_package_file(content, "package.json", &config).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource.as_deref(), Some("npm"));
    }

    #[test]
    fn extract_returns_none_when_no_deps() {
        let content = r#"{"name":"something"}"#;
        let config = JsonataExtractConfig {
            file_format: Some("json".to_owned()),
            ..Default::default()
        };
        assert!(extract_package_file(content, "test.json", &config).is_none());
    }

    #[test]
    fn extract_invalid_json_returns_none() {
        let content = "not json";
        let config = JsonataExtractConfig {
            file_format: Some("json".to_owned()),
            ..Default::default()
        };
        assert!(extract_package_file(content, "test.json", &config).is_none());
    }

    #[test]
    fn extract_from_yaml() {
        let content = "- depName: nginx\n  currentValue: 1.23.0\n";
        let config = JsonataExtractConfig {
            file_format: Some("yaml".to_owned()),
            ..Default::default()
        };
        let deps = extract_package_file(content, "test.yaml", &config).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name.as_deref(), Some("nginx"));
    }

    #[test]
    fn extract_from_toml() {
        let content = r#"[[package]]
depName = "serde"
currentValue = "1.0.0"
"#;
        let config = JsonataExtractConfig {
            file_format: Some("toml".to_owned()),
            ..Default::default()
        };
        let deps = extract_package_file(content, "test.toml", &config).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name.as_deref(), Some("serde"));
    }
}

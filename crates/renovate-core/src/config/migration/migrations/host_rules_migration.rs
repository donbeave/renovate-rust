use serde_json::Map;
use serde_json::Value;
use std::collections::BTreeSet;

use crate::config::migration::Migration;

fn migrate_datasource(value: &str) -> &str {
    match value {
        "adoptium-java" => "java-version",
        "dotnet" => "dotnet-version",
        "node" => "node-version",
        _ => value,
    }
}

fn massage_host_url(url: &str) -> String {
    if !url.contains("://") && (url.contains('/') || url.contains(':')) {
        format!("https://{url}")
    } else {
        url.to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct HostRulesMigration;

impl Default for HostRulesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl HostRulesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for HostRulesMigration {
    fn property_name(&self) -> &str {
        "hostRules"
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

        let new_host_rules: Vec<Value> = arr
            .iter()
            .map(|rule| {
                let Value::Object(map) = rule else {
                    return rule.clone();
                };

                validate_host_rule(map);

                let mut new_rule = Map::new();
                for (k, v) in map {
                    match k.as_str() {
                        "platform" => {
                            if let Some(s) = v.as_str() {
                                new_rule
                                    .entry("hostType".to_owned())
                                    .or_insert_with(|| Value::String(migrate_datasource(s).into()));
                            }
                        }
                        "matchHost" => {
                            if let Some(s) = v.as_str() {
                                new_rule
                                    .entry("matchHost".to_owned())
                                    .or_insert_with(|| Value::String(massage_host_url(s)));
                            }
                        }
                        "hostType" => {
                            if let Some(s) = v.as_str() {
                                new_rule
                                    .entry("hostType".to_owned())
                                    .or_insert_with(|| Value::String(migrate_datasource(s).into()));
                            }
                        }
                        "endpoint" | "host" | "baseUrl" | "hostName" | "domainName" => {
                            if let Some(s) = v.as_str() {
                                new_rule
                                    .entry("matchHost".to_owned())
                                    .or_insert_with(|| Value::String(massage_host_url(s)));
                            }
                        }
                        _ => {
                            new_rule.insert(k.clone(), v.clone());
                        }
                    }
                }
                Value::Object(new_rule)
            })
            .collect();

        migrated_config.insert("hostRules".into(), Value::Array(new_host_rules));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

fn validate_host_rule(rule: &Map<String, Value>) {
    let host_fields = [
        "matchHost",
        "hostName",
        "domainName",
        "baseUrl",
        "endpoint",
        "host",
    ];
    let mut hosts: Vec<&str> = Vec::new();
    for field in host_fields {
        if let Some(Value::String(s)) = rule.get(field) {
            hosts.push(s);
        }
    }

    if hosts.len() > 1 {
        let distinct_values: BTreeSet<&str> = hosts.into_iter().collect();
        if distinct_values.len() > 1 {
            panic!(
                "`hostRules` cannot contain more than one host-matching field - use `matchHost` only."
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::HostRulesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = HostRulesMigration::new();
        assert_eq!(m.property_name(), "hostRules");
    }

    #[test]
    fn migrates_host_rules() {
        let m = HostRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "hostRules",
            &json!([
                {
                    "hostType": "dotnet",
                    "baseUrl": "https://some.domain.com",
                    "token": "123test"
                },
                {
                    "hostType": "adoptium-java",
                    "domainName": "domain.com",
                    "token": "123test"
                },
                { "domainName": "domain.com/", "token": "123test" },
                { "hostType": "docker", "matchHost": "domain.com/", "token": "123test" },
                { "hostName": "some.domain.com", "token": "123test" },
                { "endpoint": "domain.com/", "token": "123test" },
                { "host": "some.domain.com", "token": "123test" },
                { "matchHost": "some.domain.com:8080", "token": "123test" }
            ]),
            &Map::new(),
            &mut migrated,
        );

        assert_eq!(
            migrated["hostRules"],
            json!([
                {
                    "hostType": "dotnet-version",
                    "matchHost": "https://some.domain.com",
                    "token": "123test"
                },
                {
                    "hostType": "java-version",
                    "matchHost": "domain.com",
                    "token": "123test"
                },
                {
                    "matchHost": "https://domain.com/",
                    "token": "123test"
                },
                {
                    "hostType": "docker",
                    "matchHost": "https://domain.com/",
                    "token": "123test"
                },
                { "matchHost": "some.domain.com", "token": "123test" },
                { "matchHost": "https://domain.com/", "token": "123test" },
                { "matchHost": "some.domain.com", "token": "123test" },
                { "matchHost": "https://some.domain.com:8080", "token": "123test" }
            ])
        );
    }

    #[test]
    fn migrates_duplicate_host_values() {
        let m = HostRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "hostRules",
            &json!([
                {
                    "hostType": "dotnet",
                    "baseUrl": "https://some.domain.com",
                    "matchHost": "https://some.domain.com",
                    "token": "123test"
                }
            ]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["hostRules"],
            json!([
                {
                    "hostType": "dotnet-version",
                    "matchHost": "https://some.domain.com",
                    "token": "123test"
                }
            ])
        );
    }

    #[test]
    #[should_panic(expected = "`hostRules` cannot contain more than one host-matching field")]
    fn throws_on_multiple_hosts() {
        let m = HostRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "hostRules",
            &json!([
                {
                    "matchHost": "https://some-diff.domain.com",
                    "baseUrl": "https://some.domain.com",
                    "token": "123test"
                }
            ]),
            &Map::new(),
            &mut migrated,
        );
    }
}

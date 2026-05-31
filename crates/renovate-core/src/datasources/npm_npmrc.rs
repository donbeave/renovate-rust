//! .npmrc file parsing.
//!
//! Parses .npmrc configuration into structured host and package rules.
//!
//! Renovate reference: `lib/modules/datasource/npm/npmrc.ts`

use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NpmrcError {
    #[error("parse error: {0}")]
    Parse(String),
    #[error("env-replace error")]
    EnvReplace,
}

#[derive(Debug, Clone, Default)]
pub struct NpmrcConfig {
    pub registry: Option<String>,
    pub entries: Vec<NpmrcEntry>,
}

#[derive(Debug, Clone)]
pub struct NpmrcEntry {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct NpmrcRules {
    pub host_rules: Vec<NpmrcHostRule>,
    pub package_rules: Vec<NpmrcPackageRule>,
}

#[derive(Debug, Clone)]
pub struct NpmrcHostRule {
    pub match_host: String,
    pub token: Option<String>,
    pub auth_type: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NpmrcPackageRule {
    pub match_package_names: Option<Vec<String>>,
    pub registry_urls: Vec<String>,
}

pub fn parse_npmrc(content: &str) -> Result<NpmrcConfig, NpmrcError> {
    let mut config = NpmrcConfig::default();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_owned();
            let value = line[eq_pos + 1..].trim().to_owned();

            if key == "registry" {
                config.registry = Some(value.clone());
            }

            config.entries.push(NpmrcEntry { key, value });
        }
    }

    Ok(config)
}

pub fn convert_npmrc_to_rules(config: &NpmrcConfig) -> NpmrcRules {
    let mut rules = NpmrcRules {
        host_rules: Vec::new(),
        package_rules: Vec::new(),
    };

    let mut hosts: HashMap<String, NpmrcHostRule> = HashMap::new();

    for entry in &config.entries {
        let key_parts: Vec<&str> = entry.key.rsplitn(2, ':').collect();
        let key_type = key_parts[0];
        let match_host = if key_parts.len() > 1 {
            get_match_host_from_npmrc_host(key_parts[1])
        } else {
            String::new()
        };

        let rule = hosts.entry(match_host.clone()).or_insert_with(|| NpmrcHostRule {
            match_host: match_host.clone(),
            token: None,
            auth_type: None,
            username: None,
            password: None,
        });

        match key_type {
            "_authToken" => {
                rule.token = Some(entry.value.clone());
            }
            "_auth" => {
                rule.token = Some(entry.value.clone());
                rule.auth_type = Some("Basic".to_owned());
            }
            "username" => {
                rule.username = Some(entry.value.clone());
            }
            "_password" => {
                let decoded =
                    String::from_utf8(base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &entry.value).unwrap_or_default())
                        .unwrap_or_default();
                rule.password = Some(decoded);
            }
            "registry"
                if key_parts.len() > 1
                    && !entry.value.is_empty()
                    && is_http_url(&entry.value)
                =>
            {
                let scope = key_parts[1].to_owned();
                rules.package_rules.push(NpmrcPackageRule {
                    match_package_names: Some(vec![format!("{}/**", scope)]),
                    registry_urls: vec![entry.value.clone()],
                });
            }
            _ => {}
        }
    }

    rules.host_rules = hosts.into_values().collect();

    if let Some(ref registry) = config.registry
        && is_http_url(registry)
    {
        rules.package_rules.push(NpmrcPackageRule {
            match_package_names: None,
            registry_urls: vec![registry.clone()],
        });
    }

    rules
}

fn get_match_host_from_npmrc_host(input: &str) -> String {
    if let Some(rest) = input.strip_prefix("//") {
        if rest.contains('/') {
            format!("https://{}", rest)
        } else {
            rest.to_owned()
        }
    } else {
        input.to_owned()
    }
}

fn is_http_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn resolve_registry_url(config: &NpmrcConfig, package_name: &str) -> String {
    let default = config
        .registry
        .as_deref()
        .unwrap_or("https://registry.npmjs.org");

    let rules = convert_npmrc_to_rules(config);

    for rule in &rules.package_rules {
        if let Some(ref patterns) = rule.match_package_names {
            for pattern in patterns {
                let prefix = pattern.trim_end_matches("/**");
                if package_name.starts_with(prefix) && !rule.registry_urls.is_empty() {
                    return rule.registry_urls[0].clone();
                }
            }
        }
    }

    default.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_npmrc_empty() {
        let config = parse_npmrc("").unwrap();
        assert!(config.entries.is_empty());
        assert!(config.registry.is_none());
    }

    #[test]
    fn parse_npmrc_comments_skipped() {
        let config = parse_npmrc("# comment\n; also comment\nregistry=https://example.com").unwrap();
        assert_eq!(config.entries.len(), 1);
        assert_eq!(config.registry.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn parse_npmrc_registry() {
        let config = parse_npmrc("registry=https://custom.registry.com").unwrap();
        assert_eq!(config.registry.as_deref(), Some("https://custom.registry.com"));
    }

    #[test]
    fn parse_npmrc_auth_token() {
        let config = parse_npmrc("//registry.example.com/:_authToken=abc123").unwrap();
        assert_eq!(config.entries.len(), 1);
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(
            rules.host_rules[0].token.as_deref(),
            Some("abc123")
        );
    }

    #[test]
    fn parse_npmrc_scoped_registry() {
        let config =
            parse_npmrc("@myorg:registry=https://myorg.registry.com").unwrap();
        assert_eq!(config.entries.len(), 1);
        let rules = convert_npmrc_to_rules(&config);
        assert!(!rules.package_rules.is_empty());
    }

    #[test]
    fn resolve_registry_url_default() {
        let config = NpmrcConfig::default();
        let url = resolve_registry_url(&config, "lodash");
        assert_eq!(url, "https://registry.npmjs.org");
    }

    #[test]
    fn resolve_registry_url_custom() {
        let config =
            parse_npmrc("registry=https://custom.registry.com").unwrap();
        let url = resolve_registry_url(&config, "lodash");
        assert_eq!(url, "https://custom.registry.com");
    }

    #[test]
    fn get_match_host_from_npmrc_host_double_slash() {
        assert_eq!(
            get_match_host_from_npmrc_host("//registry.example.com/"),
            "https://registry.example.com/"
        );
    }

    #[test]
    fn get_match_host_from_npmrc_host_no_path() {
        assert_eq!(
            get_match_host_from_npmrc_host("//registry.example.com"),
            "registry.example.com"
        );
    }

    #[test]
    fn parse_npmrc_blank_lines_skipped() {
        let config =
            parse_npmrc("\n\nregistry=https://example.com\n\n").unwrap();
        assert_eq!(config.entries.len(), 1);
    }
}

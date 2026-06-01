//! .npmrc file parsing.
//!
//! Parses .npmrc configuration into structured host and package rules.
//!
//! Renovate reference: `lib/modules/datasource/npm/npmrc.ts`

use std::collections::HashMap;

use base64::Engine as _;
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

#[derive(Debug, Clone, Default)]
pub struct NpmrcRules {
    pub host_rules: Vec<NpmrcHostRule>,
    pub package_rules: Vec<NpmrcPackageRule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Result of applying `set_npmrc` logic.
#[derive(Debug, Clone, Default)]
pub struct SetNpmrcResult {
    pub rules: NpmrcRules,
    /// Secrets extracted from the npmrc that should be added to the sanitizer.
    pub secrets: Vec<String>,
    /// Whether the npmrc was rejected due to localhost registry.
    pub rejected: bool,
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

/// Replace `${VAR}` expressions in a value using the provided environment map.
///
/// Mirrors `envReplace()` from `lib/modules/datasource/npm/npmrc.ts`.
pub fn env_replace(value: &str, env: &HashMap<String, String>) -> Result<String, NpmrcError> {
    // Regex: (\\*)\$\{([^}]+)\}
    // We implement this without regex to avoid adding a dependency.
    let mut result = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '$' {
            result.push(ch);
            continue;
        }
        if chars.peek() != Some(&'{') {
            result.push(ch);
            continue;
        }
        chars.next(); // consume '{'

        // Count leading backslashes
        let mut backslash_count = 0usize;
        let mut check = result.chars().rev();
        while check.next() == Some('\\') {
            backslash_count += 1;
        }
        // If odd number of backslashes, the $ is escaped
        if backslash_count % 2 == 1 {
            result.push(ch);
            result.push('{');
            continue;
        }

        let mut var_name = String::new();
        loop {
            match chars.next() {
                Some('}') => break,
                Some(c) => var_name.push(c),
                None => return Err(NpmrcError::EnvReplace),
            }
        }

        if let Some(val) = env.get(&var_name) {
            // Remove the backslashes that were meant to escape
            if backslash_count > 0 {
                let keep = backslash_count / 2;
                result.truncate(result.len() - backslash_count);
                for _ in 0..keep {
                    result.push('\\');
                }
            }
            result.push_str(val);
        } else {
            return Err(NpmrcError::EnvReplace);
        }
    }

    Ok(result)
}

/// Apply env-replace to every entry value in the config if `expose_all_env` is true.
fn env_replace_config(
    config: &mut NpmrcConfig,
    expose_all_env: bool,
    env: &HashMap<String, String>,
) {
    if !expose_all_env {
        return;
    }
    if let Some(ref mut registry) = config.registry {
        if let Ok(replaced) = env_replace(registry, env) {
            *registry = replaced;
        }
    }
    for entry in &mut config.entries {
        if let Ok(replaced) = env_replace(&entry.value, env) {
            entry.value = replaced;
        }
    }
}

/// Extract secrets that should be sanitized from the parsed config.
fn extract_secrets(config: &NpmrcConfig) -> Vec<String> {
    let mut secrets = Vec::new();

    for entry in &config.entries {
        let key_parts: Vec<&str> = entry.key.rsplitn(2, ':').collect();
        let key_type = key_parts[0];

        match key_type {
            "_auth" | "_authToken" => {
                if !entry.value.is_empty() {
                    secrets.push(entry.value.clone());
                }
            }
            "username" => {
                if !entry.value.is_empty() {
                    secrets.push(entry.value.clone());
                }
            }
            "_password" => {
                if !entry.value.is_empty() {
                    let host = if key_parts.len() > 1 {
                        Some(key_parts[1])
                    } else {
                        None
                    };
                    let decoded = String::from_utf8(
                        base64::Engine::decode(
                            &base64::engine::general_purpose::STANDARD,
                            &entry.value,
                        )
                        .unwrap_or_default(),
                    )
                    .unwrap_or_default();
                    // Find matching username for this host and add base64(username:password)
                    let mut added_combined = false;
                    if let Some(host) = host {
                        for e2 in &config.entries {
                            if e2.key == format!("{}:username", host) && !e2.value.is_empty() {
                                let combined = format!("{}:{}", e2.value, decoded);
                                secrets.push(
                                    base64::engine::general_purpose::STANDARD
                                        .encode(combined.as_bytes()),
                                );
                                added_combined = true;
                                break;
                            }
                        }
                    }
                    // Only add raw password if no username was found to combine with
                    if !added_combined {
                        secrets.push(entry.value.clone());
                    }
                }
            }
            _ => {}
        }
    }

    secrets
}

/// Check if any registry entry points to localhost (and we don't have exposeAllEnv).
fn has_localhost_registry(config: &NpmrcConfig) -> bool {
    if let Some(ref registry) = config.registry {
        if registry.contains("localhost") {
            return true;
        }
    }
    for entry in &config.entries {
        if entry.key.ends_with("registry") && entry.value.contains("localhost") {
            return true;
        }
    }
    false
}

/// Parse an npmrc string, optionally replace env vars, reject localhost registries,
/// and convert to host/package rules with extracted secrets.
///
/// Mirrors `setNpmrc()` from `lib/modules/datasource/npm/npmrc.ts`.
pub fn set_npmrc(
    input: &str,
    expose_all_env: bool,
    env: &HashMap<String, String>,
) -> Result<SetNpmrcResult, NpmrcError> {
    let mut config = parse_npmrc(input)?;

    if !expose_all_env && has_localhost_registry(&config) {
        return Ok(SetNpmrcResult {
            rules: NpmrcRules {
                host_rules: Vec::new(),
                package_rules: Vec::new(),
            },
            secrets: Vec::new(),
            rejected: true,
        });
    }

    env_replace_config(&mut config, expose_all_env, env);

    let secrets = extract_secrets(&config);
    let rules = convert_npmrc_to_rules(&config);

    Ok(SetNpmrcResult {
        rules,
        secrets,
        rejected: false,
    })
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

        match key_type {
            "_authToken" => {
                let rule = hosts
                    .entry(match_host.clone())
                    .or_insert_with(|| NpmrcHostRule {
                        match_host: match_host.clone(),
                        token: None,
                        auth_type: None,
                        username: None,
                        password: None,
                    });
                rule.token = Some(entry.value.clone());
            }
            "_auth" => {
                let rule = hosts
                    .entry(match_host.clone())
                    .or_insert_with(|| NpmrcHostRule {
                        match_host: match_host.clone(),
                        token: None,
                        auth_type: None,
                        username: None,
                        password: None,
                    });
                rule.token = Some(entry.value.clone());
                rule.auth_type = Some("Basic".to_owned());
            }
            "username" => {
                let rule = hosts
                    .entry(match_host.clone())
                    .or_insert_with(|| NpmrcHostRule {
                        match_host: match_host.clone(),
                        token: None,
                        auth_type: None,
                        username: None,
                        password: None,
                    });
                rule.username = Some(entry.value.clone());
            }
            "_password" => {
                let rule = hosts
                    .entry(match_host.clone())
                    .or_insert_with(|| NpmrcHostRule {
                        match_host: match_host.clone(),
                        token: None,
                        auth_type: None,
                        username: None,
                        password: None,
                    });
                let decoded = String::from_utf8(
                    base64::Engine::decode(
                        &base64::engine::general_purpose::STANDARD,
                        &entry.value,
                    )
                    .unwrap_or_default(),
                )
                .unwrap_or_default();
                rule.password = Some(decoded);
            }
            "registry"
                if key_parts.len() > 1 && !entry.value.is_empty() && is_http_url(&entry.value) =>
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

/// Look up the Authorization header value for a given registry URL from
/// converted npmrc rules.
///
/// Returns `Some("Bearer <token>")` for `_authToken` entries,
/// `Some("Basic <base64>")` for `_auth` / username+password entries.
pub fn auth_header_for_registry(registry: &str, rules: &NpmrcRules) -> Option<String> {
    // Normalize registry URL for matching.
    let registry = registry.trim_end_matches('/');

    for rule in &rules.host_rules {
        let match_host = rule.match_host.trim_end_matches('/');
        // Match by exact host, or by prefix for https://host/path rules.
        if match_host.is_empty()
            || registry.starts_with(match_host)
            || match_host.starts_with(registry)
        {
            if let Some(ref token) = rule.token {
                if rule.auth_type.as_deref() == Some("Basic") {
                    return Some(format!("Basic {}", token));
                }
                return Some(format!("Bearer {}", token));
            }
            if let (Some(username), Some(password)) = (&rule.username, &rule.password) {
                let combined = format!("{}:{}", username, password);
                let b64 = base64::engine::general_purpose::STANDARD.encode(combined.as_bytes());
                return Some(format!("Basic {}", b64));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── parse_npmrc ───────────────────────────────────────────────────────────

    #[test]
    fn parse_npmrc_empty() {
        let config = parse_npmrc("").unwrap();
        assert!(config.entries.is_empty());
        assert!(config.registry.is_none());
    }

    #[test]
    fn parse_npmrc_comments_skipped() {
        let config =
            parse_npmrc("# comment\n; also comment\nregistry=https://example.com").unwrap();
        assert_eq!(config.entries.len(), 1);
        assert_eq!(config.registry.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn parse_npmrc_registry() {
        let config = parse_npmrc("registry=https://custom.registry.com").unwrap();
        assert_eq!(
            config.registry.as_deref(),
            Some("https://custom.registry.com")
        );
    }

    #[test]
    fn parse_npmrc_auth_token() {
        let config = parse_npmrc("//registry.example.com/:_authToken=abc123").unwrap();
        assert_eq!(config.entries.len(), 1);
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(rules.host_rules[0].token.as_deref(), Some("abc123"));
    }

    #[test]
    fn parse_npmrc_scoped_registry() {
        let config = parse_npmrc("@myorg:registry=https://myorg.registry.com").unwrap();
        assert_eq!(config.entries.len(), 1);
        let rules = convert_npmrc_to_rules(&config);
        assert!(!rules.package_rules.is_empty());
    }

    #[test]
    fn parse_npmrc_blank_lines_skipped() {
        let config = parse_npmrc("\n\nregistry=https://example.com\n\n").unwrap();
        assert_eq!(config.entries.len(), 1);
    }

    // ── resolve_registry_url ──────────────────────────────────────────────────

    #[test]
    fn resolve_registry_url_default() {
        let config = NpmrcConfig::default();
        let url = resolve_registry_url(&config, "lodash");
        assert_eq!(url, "https://registry.npmjs.org");
    }

    #[test]
    fn resolve_registry_url_custom() {
        let config = parse_npmrc("registry=https://custom.registry.com").unwrap();
        let url = resolve_registry_url(&config, "lodash");
        assert_eq!(url, "https://custom.registry.com");
    }

    // ── get_match_host_from_npmrc_host ────────────────────────────────────────

    // Ported: "parses //host/path" — lib/modules/datasource/npm/npmrc.spec.ts line 28
    #[test]
    fn get_match_host_from_npmrc_host_double_slash() {
        assert_eq!(
            get_match_host_from_npmrc_host("//registry.example.com/"),
            "https://registry.example.com/"
        );
    }

    // Ported: "parses //host" — lib/modules/datasource/npm/npmrc.spec.ts line 22
    #[test]
    fn get_match_host_from_npmrc_host_no_path() {
        assert_eq!(
            get_match_host_from_npmrc_host("//registry.example.com"),
            "registry.example.com"
        );
    }

    // Ported: "parses https://host" — lib/modules/datasource/npm/npmrc.spec.ts line 34
    #[test]
    fn get_match_host_from_npmrc_host_https() {
        assert_eq!(
            get_match_host_from_npmrc_host("https://registry.example.com"),
            "https://registry.example.com"
        );
    }

    // ── convert_npmrc_to_rules ────────────────────────────────────────────────

    // Ported: "rejects invalid registries" — lib/modules/datasource/npm/npmrc.spec.ts line 42
    #[test]
    fn convert_npmrc_rejects_invalid_registries() {
        let config = parse_npmrc("registry=1\n@scope:registry=2\n").unwrap();
        let rules = convert_npmrc_to_rules(&config);
        assert!(rules.host_rules.is_empty());
        assert!(rules.package_rules.is_empty());
    }

    // Ported: "handles naked auth" — lib/modules/datasource/npm/npmrc.spec.ts line 50
    #[test]
    fn convert_npmrc_handles_naked_auth() {
        let config = parse_npmrc("_auth=YWRtaW46YWRtaW4=\n").unwrap();
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(
            rules.host_rules[0],
            NpmrcHostRule {
                match_host: String::new(),
                token: Some("YWRtaW46YWRtaW4=".to_owned()),
                auth_type: Some("Basic".to_owned()),
                username: None,
                password: None,
            }
        );
    }

    // Ported: "handles host, path and auth" — lib/modules/datasource/npm/npmrc.spec.ts line 66
    #[test]
    fn convert_npmrc_handles_host_path_and_auth() {
        let config = parse_npmrc("//my-registry.example.com/:_authToken=123test==\n").unwrap();
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(
            rules.host_rules[0],
            NpmrcHostRule {
                match_host: "https://my-registry.example.com/".to_owned(),
                token: Some("123test==".to_owned()),
                auth_type: None,
                username: None,
                password: None,
            }
        );
    }

    // Ported: "handles host, path, port and auth" — lib/modules/datasource/npm/npmrc.spec.ts line 84
    #[test]
    fn convert_npmrc_handles_host_path_port_and_auth() {
        let config =
            parse_npmrc("//my-registry.example.com:4040/some-path/:_authToken=123test==\n")
                .unwrap();
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(
            rules.host_rules[0],
            NpmrcHostRule {
                match_host: "https://my-registry.example.com:4040/some-path/".to_owned(),
                token: Some("123test==".to_owned()),
                auth_type: None,
                username: None,
                password: None,
            }
        );
    }

    // Ported: "handles naked authToken" — lib/modules/datasource/npm/npmrc.spec.ts line 103
    #[test]
    fn convert_npmrc_handles_naked_auth_token() {
        let config = parse_npmrc("_authToken=abc123\n").unwrap();
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(
            rules.host_rules[0],
            NpmrcHostRule {
                match_host: String::new(),
                token: Some("abc123".to_owned()),
                auth_type: None,
                username: None,
                password: None,
            }
        );
    }

    // Ported: "handles host authToken" — lib/modules/datasource/npm/npmrc.spec.ts line 118
    #[test]
    fn convert_npmrc_handles_host_auth_token() {
        let config = parse_npmrc(
            "@fontawesome:registry=https://npm.fontawesome.com/\n//npm.fontawesome.com/:_authToken=abc123",
        )
        .unwrap();
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(
            rules.host_rules[0],
            NpmrcHostRule {
                match_host: "https://npm.fontawesome.com/".to_owned(),
                token: Some("abc123".to_owned()),
                auth_type: None,
                username: None,
                password: None,
            }
        );
        assert_eq!(rules.package_rules.len(), 1);
        assert_eq!(
            rules.package_rules[0].match_package_names,
            Some(vec!["@fontawesome/**".to_owned()])
        );
        assert_eq!(
            rules.package_rules[0].registry_urls,
            vec!["https://npm.fontawesome.com/".to_owned()]
        );
    }

    // Ported: "handles username and _password" — lib/modules/datasource/npm/npmrc.spec.ts line 151
    #[test]
    fn convert_npmrc_handles_username_and_password() {
        let config = parse_npmrc(
            "//my-registry.example.com/npm-private/:_password=dGVzdA==\n//my-registry.example.com/npm-private/:username=bot\n//my-registry.example.com/npm-private/:always-auth=true",
        )
        .unwrap();
        let rules = convert_npmrc_to_rules(&config);
        assert_eq!(rules.host_rules.len(), 1);
        assert_eq!(
            rules.host_rules[0],
            NpmrcHostRule {
                match_host: "https://my-registry.example.com/npm-private/".to_owned(),
                token: None,
                auth_type: None,
                username: Some("bot".to_owned()),
                password: Some("test".to_owned()),
            }
        );
        assert!(rules.package_rules.is_empty());
    }

    // ── env_replace ───────────────────────────────────────────────────────────

    #[test]
    fn env_replace_basic() {
        let mut env = HashMap::new();
        env.insert("NPM_TOKEN".to_owned(), "secret123".to_owned());
        assert_eq!(
            env_replace("token=${NPM_TOKEN}", &env).unwrap(),
            "token=secret123"
        );
    }

    #[test]
    fn env_replace_multiple() {
        let mut env = HashMap::new();
        env.insert("A".to_owned(), "1".to_owned());
        env.insert("B".to_owned(), "2".to_owned());
        assert_eq!(env_replace("${A}-${B}", &env).unwrap(), "1-2");
    }

    #[test]
    fn env_replace_missing_var_errors() {
        let env: HashMap<String, String> = HashMap::new();
        assert!(env_replace("${MISSING}", &env).is_err());
    }

    // ── set_npmrc (sanitization + localhost) ──────────────────────────────────

    // Ported: "sanitize _auth" — lib/modules/datasource/npm/npmrc.spec.ts line 174
    #[test]
    fn set_npmrc_sanitize_auth() {
        let env: HashMap<String, String> = HashMap::new();
        let result = set_npmrc("_auth=test\n", false, &env).unwrap();
        assert!(result.secrets.contains(&"test".to_owned()));
    }

    // Ported: "sanitize _authtoken" — lib/modules/datasource/npm/npmrc.spec.ts line 181
    #[test]
    fn set_npmrc_sanitize_auth_token() {
        let env: HashMap<String, String> = HashMap::new();
        let result = set_npmrc(
            "//registry.test.com:_authToken=test\n_authToken=${NPM_TOKEN}\n",
            false,
            &env,
        )
        .unwrap();
        assert!(result.secrets.contains(&"test".to_owned()));
        assert!(result.secrets.contains(&"${NPM_TOKEN}".to_owned()));
        assert_eq!(result.secrets.len(), 2);
    }

    // Ported: "sanitize _password" — lib/modules/datasource/npm/npmrc.spec.ts line 191
    #[test]
    fn set_npmrc_sanitize_password() {
        let env: HashMap<String, String> = HashMap::new();
        let result = set_npmrc(
            "registry=https://test.org\n//test.org/:username=test\n//test.org/:_password=dGVzdA==\n",
            false,
            &env,
        )
        .unwrap();
        assert!(result.secrets.contains(&"test".to_owned()));
        // dGVzdA== decodes to "test"; base64("test:test") = "dGVzdDp0ZXN0"
        assert!(result.secrets.contains(&"dGVzdDp0ZXN0".to_owned()));
        assert_eq!(result.secrets.len(), 2);
    }

    // Ported: "sanitize _authtoken with high trust" — lib/modules/datasource/npm/npmrc.spec.ts line 203
    #[test]
    fn set_npmrc_sanitize_auth_token_with_high_trust() {
        let mut env = HashMap::new();
        env.insert("TEST_TOKEN".to_owned(), "test".to_owned());
        let result = set_npmrc(
            "//registry.test.com:_authToken=${TEST_TOKEN}\n_authToken=\nregistry=http://localhost\n",
            true,
            &env,
        )
        .unwrap();
        // With exposeAllEnv=true, env replacement happens before secret extraction
        assert!(result.secrets.contains(&"test".to_owned()));
        // The empty _authToken should not add a secret
        assert_eq!(result.secrets.len(), 1);
    }

    // Ported: "ignores localhost" — lib/modules/datasource/npm/npmrc.spec.ts line 214
    #[test]
    fn set_npmrc_ignores_localhost() {
        let env: HashMap<String, String> = HashMap::new();
        let result = set_npmrc("registry=http://localhost\n", false, &env).unwrap();
        assert!(result.rejected);
        assert!(result.secrets.is_empty());
    }

    // ── auth_header_for_registry ──────────────────────────────────────────────

    #[test]
    fn auth_header_bearer_from_auth_token() {
        let rules = NpmrcRules {
            host_rules: vec![NpmrcHostRule {
                match_host: "https://registry.example.com".to_owned(),
                token: Some("abc123".to_owned()),
                auth_type: None,
                username: None,
                password: None,
            }],
            package_rules: vec![],
        };
        assert_eq!(
            auth_header_for_registry("https://registry.example.com/", &rules),
            Some("Bearer abc123".to_owned())
        );
    }

    #[test]
    fn auth_header_basic_from_auth() {
        let rules = NpmrcRules {
            host_rules: vec![NpmrcHostRule {
                match_host: "https://registry.example.com".to_owned(),
                token: Some("dXNlcjpwYXNz".to_owned()),
                auth_type: Some("Basic".to_owned()),
                username: None,
                password: None,
            }],
            package_rules: vec![],
        };
        assert_eq!(
            auth_header_for_registry("https://registry.example.com/", &rules),
            Some("Basic dXNlcjpwYXNz".to_owned())
        );
    }

    #[test]
    fn auth_header_basic_from_username_password() {
        let rules = NpmrcRules {
            host_rules: vec![NpmrcHostRule {
                match_host: "https://registry.example.com".to_owned(),
                token: None,
                auth_type: None,
                username: Some("bot".to_owned()),
                password: Some("secret".to_owned()),
            }],
            package_rules: vec![],
        };
        assert_eq!(
            auth_header_for_registry("https://registry.example.com/", &rules),
            Some("Basic Ym90OnNlY3JldA==".to_owned())
        );
    }

    #[test]
    fn auth_header_no_match_returns_none() {
        let rules = NpmrcRules {
            host_rules: vec![NpmrcHostRule {
                match_host: "https://other.example.com".to_owned(),
                token: Some("abc123".to_owned()),
                auth_type: None,
                username: None,
                password: None,
            }],
            package_rules: vec![],
        };
        assert_eq!(
            auth_header_for_registry("https://registry.example.com/", &rules),
            None
        );
    }
}

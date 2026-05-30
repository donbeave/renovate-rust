use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HostRule {
    #[serde(default)]
    pub match_host: Option<String>,
    #[serde(default)]
    pub host_type: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub abort_on_error: Option<bool>,
    #[serde(default)]
    pub abort_ignore_status_codes: Option<Vec<u16>>,
    #[serde(default)]
    pub timeout: Option<u64>,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(default)]
    pub concurrent_request_limit: Option<usize>,
    #[serde(default)]
    pub max_requests_per_second: Option<u64>,
    #[serde(default)]
    pub https_certificate_authority: Option<String>,
    #[serde(default)]
    pub https_private_key: Option<String>,
    #[serde(default)]
    pub https_certificate: Option<String>,
}

const GITHUB_API_USING_HOST_TYPES: &[&str] = &[
    "github-releases",
    "github-release-attachments",
    "github-tags",
    "pod",
    "hermit",
    "github-changelog",
    "conan",
];

const GITLAB_API_USING_HOST_TYPES: &[&str] = &[
    "gitlab-releases",
    "gitlab-tags",
    "gitlab-packages",
    "gitlab-changelog",
    "pypi",
];

const BITBUCKET_API_USING_HOST_TYPES: &[&str] = &["bitbucket-changelog"];

const BITBUCKET_SERVER_API_USING_HOST_TYPES: &[&str] = &["bitbucket-server-changelog"];

const FORGEJO_API_USING_HOST_TYPES: &[&str] = &[
    "forgejo-changelog",
    "forgejo-releases",
    "forgejo-tags",
];

const GITEA_API_USING_HOST_TYPES: &[&str] = &["gitea-changelog", "gitea-releases", "gitea-tags"];

pub fn find_matching_rule(
    url: &str,
    host_type: Option<&str>,
    host_rules: &[HostRule],
    platform: Option<&str>,
    platform_endpoint: Option<&str>,
) -> HostRule {
    let mut merged = HostRule::default();

    let primary = find_rules(url, host_type, host_rules);
    merge_rule_into(&mut merged, &primary);

    if merged.token.is_some()
        || merged.username.is_some()
        || merged.password.is_some()
    {
        return merged;
    }

    if let Some(ht) = host_type
        && let Some(fallback_ht) = get_platform_fallback_host_type(ht, url, platform, platform_endpoint) {
            let fallback = find_rules(url, Some(fallback_ht), host_rules);
            merge_rule_into(&mut merged, &fallback);
        }

    merged
}

fn find_rules(url: &str, host_type: Option<&str>, host_rules: &[HostRule]) -> HostRule {
    let mut best: Option<HostRule> = None;

    for rule in host_rules {
        let host_match = match rule.match_host {
            Some(ref pattern) => host_matches(url, pattern),
            None => false,
        };
        let type_match = match (host_type, rule.host_type.as_deref()) {
            (Some(ht), Some(rht)) => ht == rht,
            _ => false,
        };

        if host_match || type_match {
            best = Some(rule.clone());
        }
    }

    best.unwrap_or_default()
}

fn merge_rule_into(target: &mut HostRule, source: &HostRule) {
    if source.token.is_some() {
        target.token = source.token.clone();
    }
    if source.username.is_some() {
        target.username = source.username.clone();
    }
    if source.password.is_some() {
        target.password = source.password.clone();
    }
    if source.enabled.is_some() {
        target.enabled = source.enabled;
    }
    if source.abort_on_error.is_some() {
        target.abort_on_error = source.abort_on_error;
    }
    if source.abort_ignore_status_codes.is_some() {
        target.abort_ignore_status_codes = source.abort_ignore_status_codes.clone();
    }
    if source.timeout.is_some() {
        target.timeout = source.timeout;
    }
    if source.concurrent_request_limit.is_some() {
        target.concurrent_request_limit = source.concurrent_request_limit;
    }
    if source.max_requests_per_second.is_some() {
        target.max_requests_per_second = source.max_requests_per_second;
    }
    if source.https_certificate_authority.is_some() {
        target.https_certificate_authority = source.https_certificate_authority.clone();
    }
    if source.https_private_key.is_some() {
        target.https_private_key = source.https_private_key.clone();
    }
    if source.https_certificate.is_some() {
        target.https_certificate = source.https_certificate.clone();
    }
    if source.headers.is_some()
        && let Some(ref h) = source.headers {
            let target_headers = target.headers.get_or_insert_with(HashMap::new);
            for (k, v) in h {
                target_headers.insert(k.clone(), v.clone());
            }
        }
}

fn get_platform_fallback_host_type<'a>(
    host_type: &'a str,
    url: &str,
    platform: Option<&str>,
    platform_endpoint: Option<&str>,
) -> Option<&'a str> {
    if GITHUB_API_USING_HOST_TYPES.contains(&host_type) {
        return Some("github");
    }
    if url.starts_with("https://api.github.com/") {
        return Some("github");
    }
    if platform == Some("github")
        && let Some(endpoint) = platform_endpoint
            && let Ok(parsed_url) = url::Url::parse(url)
                && let Some(host) = parsed_url.host_str()
                    && let Ok(endpoint_parsed) = url::Url::parse(endpoint)
                        && endpoint_parsed.host_str() == Some(host) {
                            return Some("github");
                        }
    if GITLAB_API_USING_HOST_TYPES.contains(&host_type) {
        return Some("gitlab");
    }
    if BITBUCKET_API_USING_HOST_TYPES.contains(&host_type) {
        return Some("bitbucket");
    }
    if BITBUCKET_SERVER_API_USING_HOST_TYPES.contains(&host_type) {
        return Some("bitbucket-server");
    }
    if FORGEJO_API_USING_HOST_TYPES.contains(&host_type) {
        return Some("forgejo");
    }
    if GITEA_API_USING_HOST_TYPES.contains(&host_type) {
        return Some("gitea");
    }
    None
}

fn host_matches(url: &str, pattern: &str) -> bool {
    let Some(host) = url::Url::parse(url).ok().and_then(|u| u.host_str().map(|h| h.to_owned())) else {
        return false;
    };

    if host == pattern {
        return true;
    }
    if pattern.starts_with("http://") || pattern.starts_with("https://") {
        return url.starts_with(pattern);
    }
    if pattern.starts_with('/') {
        return url::Url::parse(url)
            .map(|u| u.path().starts_with(pattern))
            .unwrap_or(false);
    }
    if pattern.starts_with('*') && host.ends_with(&pattern[1..]) {
        return true;
    }
    if pattern.starts_with('.') && (host.ends_with(pattern) || host == pattern[1..]) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: host_rules behavior test
    #[test]
    fn host_matches_exact() {
        assert!(host_matches("https://api.github.com/repos", "api.github.com"));
    }

    // Rust-specific: host_rules behavior test
    #[test]
    fn host_matches_url_prefix() {
        assert!(host_matches(
            "https://api.github.com/repos",
            "https://api.github.com"
        ));
    }

    // Rust-specific: host_rules behavior test
    #[test]
    fn host_matches_wildcard() {
        assert!(host_matches(
            "https://ghe.example.com/api",
            "*.example.com"
        ));
    }

    // Rust-specific: host_rules behavior test
    #[test]
    fn host_no_match() {
        assert!(!host_matches("https://gitlab.com/api", "api.github.com"));
    }

    // Rust-specific: host_rules behavior test
    #[test]
    fn find_matching_rule_by_host_type() {
        let rules = vec![HostRule {
            host_type: Some("github".to_owned()),
            token: Some("gh_token".to_owned()),
            ..Default::default()
        }];
        let result = find_matching_rule(
            "https://api.github.com/repos",
            Some("github"),
            &rules,
            None,
            None,
        );
        assert_eq!(result.token.as_deref(), Some("gh_token"));
    }

    // Rust-specific: host_rules behavior test
    #[test]
    fn find_matching_rule_fallback() {
        let rules = vec![HostRule {
            host_type: Some("github".to_owned()),
            token: Some("fallback_token".to_owned()),
            ..Default::default()
        }];
        let result = find_matching_rule(
            "https://api.github.com/repos",
            Some("github-releases"),
            &rules,
            None,
            None,
        );
        assert_eq!(result.token.as_deref(), Some("fallback_token"));
    }

    // Rust-specific: host_rules behavior test
    #[test]
    fn find_matching_rule_no_match() {
        let rules = vec![];
        let result = find_matching_rule(
            "https://example.com/api",
            Some("npm"),
            &rules,
            None,
            None,
        );
        assert!(result.token.is_none());
    }

    // Rust-specific: host_rules behavior test
    #[test]
    fn merge_rule_priority() {
        let rules = vec![
            HostRule {
                host_type: Some("github-releases".to_owned()),
                token: Some("specific_token".to_owned()),
                ..Default::default()
            },
            HostRule {
                host_type: Some("github".to_owned()),
                token: Some("generic_token".to_owned()),
                ..Default::default()
            },
        ];
        let result = find_matching_rule(
            "https://api.github.com/repos",
            Some("github-releases"),
            &rules,
            None,
            None,
        );
        assert_eq!(result.token.as_deref(), Some("specific_token"));
    }
}

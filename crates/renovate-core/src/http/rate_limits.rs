use std::sync::LazyLock;

use regex::Regex;

static CONCURRENCY_DEFAULTS: LazyLock<Vec<(Regex, usize)>> = LazyLock::new(|| {
    vec![
        (Regex::new(r"registry\.npmjs\.org").unwrap(), 999),
        (Regex::new(r"repology\.org").unwrap(), 1),
        (Regex::new(r"packages\.typst\.org").unwrap(), 1),
    ]
});

static THROTTLE_DEFAULTS: LazyLock<Vec<(Regex, u64)>> = LazyLock::new(|| {
    vec![
        (Regex::new(r"rubygems\.org").unwrap(), 125),
        (Regex::new(r"crates\.io/api/").unwrap(), 1000),
        (Regex::new(r"plugins\.gradle\.org").unwrap(), 50),
        (Regex::new(r"repology\.org").unwrap(), 2000),
    ]
});

static DEFAULT_CONCURRENCY: usize = 16;

#[derive(Debug, Clone, Default)]
pub struct ConcurrencyLimitRule {
    pub match_host: String,
    pub concurrency: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ThrottleLimitRule {
    pub match_host: String,
    pub throttle_ms: u64,
}

pub fn get_concurrent_requests_limit(
    url: &str,
    host_rules_limits: &[ConcurrencyLimitRule],
) -> Option<usize> {
    let host = extract_host(url)?;

    let mut rule_limit: Option<usize> = None;
    for rule in host_rules_limits {
        if host_matches(&host, &rule.match_host) {
            rule_limit = Some(rule.concurrency);
        }
    }

    let mut default_limit: Option<usize> = None;
    for (re, limit) in CONCURRENCY_DEFAULTS.iter() {
        if re.is_match(url) {
            default_limit = Some(*limit);
        }
    }

    match (rule_limit, default_limit) {
        (Some(r), Some(d)) => Some(r.min(d)),
        (Some(r), None) => Some(r),
        (None, Some(d)) => Some(d),
        (None, None) => Some(DEFAULT_CONCURRENCY),
    }
}

pub fn get_throttle_interval_ms(
    url: &str,
    host_rules_limits: &[ThrottleLimitRule],
) -> Option<u64> {
    let host = extract_host(url)?;

    let mut rule_limit: Option<u64> = None;
    for rule in host_rules_limits {
        if host_matches(&host, &rule.match_host) {
            rule_limit = Some(rule.throttle_ms);
        }
    }

    let mut default_limit: Option<u64> = None;
    for (re, interval) in THROTTLE_DEFAULTS.iter() {
        if re.is_match(url) {
            default_limit = Some(*interval);
        }
    }

    match (rule_limit, default_limit) {
        (Some(r), Some(d)) => Some(r.max(d)),
        (Some(r), None) => Some(r),
        (None, Some(d)) => Some(d),
        (None, None) => None,
    }
}

fn extract_host(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_owned()))
}

fn host_matches(host: &str, pattern: &str) -> bool {
    if host == pattern {
        return true;
    }
    if pattern.starts_with('*') && host.ends_with(&pattern[1..]) {
        return true;
    }
    if pattern.starts_with('.') && (host.ends_with(pattern) || host == &pattern[1..]) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_host_from_url() {
        assert_eq!(
            extract_host("https://registry.npmjs.org/foo"),
            Some("registry.npmjs.org".to_owned())
        );
        assert_eq!(
            extract_host("https://api.github.com/repos"),
            Some("api.github.com".to_owned())
        );
        assert_eq!(extract_host("not-a-url"), None);
    }

    #[test]
    fn concurrency_npmjs() {
        let limit =
            get_concurrent_requests_limit("https://registry.npmjs.org/foo", &[]);
        assert_eq!(limit, Some(999));
    }

    #[test]
    fn concurrency_repology() {
        let limit =
            get_concurrent_requests_limit("https://repology.org/api/v1/project/foo", &[]);
        assert_eq!(limit, Some(1));
    }

    #[test]
    fn concurrency_default() {
        let limit =
            get_concurrent_requests_limit("https://example.com/api", &[]);
        assert_eq!(limit, Some(16));
    }

    #[test]
    fn throttle_rubygems() {
        let interval =
            get_throttle_interval_ms("https://rubygems.org/api/v1/gems/foo.json", &[]);
        assert_eq!(interval, Some(125));
    }

    #[test]
    fn throttle_crates_io() {
        let interval =
            get_throttle_interval_ms("https://crates.io/api/v1/crates/foo", &[]);
        assert_eq!(interval, Some(1000));
    }

    #[test]
    fn throttle_none_for_unknown() {
        let interval =
            get_throttle_interval_ms("https://example.com/api", &[]);
        assert!(interval.is_none());
    }

    #[test]
    fn host_matches_exact() {
        assert!(host_matches("api.github.com", "api.github.com"));
        assert!(!host_matches("api.github.com", "github.com"));
    }

    #[test]
    fn host_matches_wildcard() {
        assert!(host_matches("api.github.com", "*.github.com"));
        assert!(!host_matches("github.com", "*.github.com"));
    }

    #[test]
    fn host_matches_dot_prefix() {
        assert!(host_matches("api.github.com", ".github.com"));
        assert!(host_matches("github.com", ".github.com"));
    }

    #[test]
    fn concurrency_rule_limits_min_with_default() {
        let rules = vec![ConcurrencyLimitRule {
            match_host: "registry.npmjs.org".to_owned(),
            concurrency: 5,
        }];
        let limit =
            get_concurrent_requests_limit("https://registry.npmjs.org/foo", &rules);
        assert_eq!(limit, Some(5));
    }

    #[test]
    fn throttle_rule_limits_max_with_default() {
        let rules = vec![ThrottleLimitRule {
            match_host: "rubygems.org".to_owned(),
            throttle_ms: 500,
        }];
        let interval =
            get_throttle_interval_ms("https://rubygems.org/api/v1/gems/foo.json", &rules);
        assert_eq!(interval, Some(500));
    }
}

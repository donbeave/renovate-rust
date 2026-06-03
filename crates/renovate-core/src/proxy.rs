//! HTTP proxy bootstrap utilities.
//!
//! Mirrors `lib/proxy.ts` from the Renovate TypeScript reference.

use std::collections::HashMap;

/// Determine whether an HTTP/HTTPS proxy is configured.
///
/// Checks `HTTP_PROXY` and `HTTPS_PROXY` (and their lowercase variants) from
/// the provided environment map and returns `true` when either is non-empty.
///
/// Unlike the TypeScript version this function is pure (no global state) so
/// that callers can test it deterministically.
///
/// Mirrors `bootstrap()` + `hasProxy()` from `lib/proxy.ts`.
pub fn has_proxy_in_env(env: &HashMap<String, String>) -> bool {
    let http = env.get("HTTP_PROXY").or_else(|| env.get("http_proxy"));
    let https = env.get("HTTPS_PROXY").or_else(|| env.get("https_proxy"));
    let non_empty = |s: &&String| !s.is_empty();
    http.filter(non_empty).is_some() || https.filter(non_empty).is_some()
}

/// Copy uppercase proxy env vars to their lowercase counterparts (and vice
/// versa) and return whether a proxy is configured.
///
/// This normalises the environment so that tools that only read lowercase or
/// only read uppercase proxy variables both see the value.
pub fn bootstrap(env: &mut HashMap<String, String>) -> bool {
    for upper in &["HTTP_PROXY", "HTTPS_PROXY", "NO_PROXY"] {
        let lower = upper.to_lowercase();
        // Uppercase missing but lowercase present → copy up
        if !env.contains_key(*upper)
            && let Some(v) = env.get(lower.as_str()).cloned()
        {
            env.insert((*upper).to_owned(), v);
        }
        // Uppercase present → copy down
        if let Some(v) = env.get(*upper).cloned()
            && !v.is_empty()
        {
            env.insert(lower.clone(), v);
        }
    }
    has_proxy_in_env(env)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    // Ported: "respects HTTP_PROXY" — lib/proxy.spec.ts line 17
    #[test]
    fn respects_http_proxy() {
        let mut e = env(&[("HTTP_PROXY", "http://example.org/http-proxy")]);
        assert!(bootstrap(&mut e));
    }

    // Ported: "copies upper case HTTP_PROXY to http_proxy" — lib/proxy.spec.ts line 23
    #[test]
    fn copies_upper_http_proxy_to_lower() {
        let mut e = env(&[("HTTP_PROXY", "http://example.org/http-proxy")]);
        bootstrap(&mut e);
        assert!(e.contains_key("HTTP_PROXY"));
        assert!(e.contains_key("http_proxy"));
        assert!(!e.contains_key("HTTPS_PROXY"));
        assert!(!e.contains_key("https_proxy"));
        assert!(!e.contains_key("NO_PROXY"));
        assert!(!e.contains_key("no_proxy"));
    }

    // Ported: "respects HTTPS_PROXY" — lib/proxy.spec.ts line 36
    #[test]
    fn respects_https_proxy() {
        let mut e = env(&[("HTTPS_PROXY", "http://example.org/https-proxy")]);
        assert!(bootstrap(&mut e));
    }

    // Ported: "copies upper case HTTPS_PROXY to https_proxy" — lib/proxy.spec.ts line 42
    #[test]
    fn copies_upper_https_proxy_to_lower() {
        let mut e = env(&[("HTTPS_PROXY", "http://example.org/https-proxy")]);
        bootstrap(&mut e);
        assert!(e.contains_key("HTTPS_PROXY"));
        assert!(e.contains_key("https_proxy"));
        assert!(!e.contains_key("HTTP_PROXY"));
        assert!(!e.contains_key("http_proxy"));
        assert!(!e.contains_key("NO_PROXY"));
        assert!(!e.contains_key("no_proxy"));
    }

    // Ported: "does nothing" — lib/proxy.spec.ts line 55
    #[test]
    fn does_nothing_with_only_no_proxy() {
        let mut e = env(&[("no_proxy", "http://example.org/no-proxy")]);
        assert!(!bootstrap(&mut e));
    }

    #[test]
    fn has_proxy_in_env_detects() {
        let mut e = HashMap::new();
        e.insert("HTTP_PROXY".to_owned(), "http://proxy".to_owned());
        assert!(has_proxy_in_env(&e));
        let mut e2 = HashMap::new();
        e2.insert("NO_PROXY".to_owned(), "http://noproxy".to_owned());
        assert!(!has_proxy_in_env(&e2));
    }
}

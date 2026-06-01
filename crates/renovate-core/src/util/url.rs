//! URL utilities — mirrors `lib/util/url.ts`.

/// Join URL path segments, ensuring exactly one `/` between each.
///
/// Leading/trailing slashes on individual segments are normalized so
/// the result always has exactly one `/` separator.
pub fn join_url(parts: &[&str]) -> String {
    if parts.is_empty() {
        return String::new();
    }

    let base = parts[0].trim_end_matches('/');
    let rest: Vec<&str> = parts[1..]
        .iter()
        .map(|p| p.trim_matches('/'))
        .filter(|p| !p.is_empty())
        .collect();

    if rest.is_empty() {
        base.to_owned()
    } else {
        format!("{base}/{}", rest.join("/"))
    }
}

/// Remove trailing `/` from a URL string.
pub fn trim_trailing_slash(url: &str) -> String {
    let trimmed = url.trim_end_matches('/');
    if trimmed.is_empty() {
        return "/".to_owned();
    }
    if url.contains("://") {
        let scheme_end = url.find("://").unwrap_or(0);
        if scheme_end > 0 && trimmed.len() <= scheme_end + 3 {
            return format!("{trimmed}//");
        }
    }
    trimmed.to_owned()
}

/// Extract the domain (host) from a URL.
pub fn get_domain(url: &str) -> Option<String> {
    let after_scheme = if url.contains("://") {
        url.split("://").nth(1)?
    } else {
        url
    };
    let host_port = after_scheme.split('/').next()?;
    let host = host_port.split(':').next()?;
    if host.is_empty() {
        None
    } else {
        Some(host.to_owned())
    }
}

/// Validate that a URL has a valid format (scheme + host).
pub fn validate_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }
    let Ok(parsed) = url::Url::parse(url) else {
        return false;
    };
    parsed.host_str().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_url_basic() {
        assert_eq!(
            join_url(&["https://example.com", "api", "v1"]),
            "https://example.com/api/v1"
        );
    }

    #[test]
    fn join_url_single_part() {
        assert_eq!(join_url(&["https://example.com"]), "https://example.com");
    }

    #[test]
    fn join_url_empty() {
        assert_eq!(join_url(&[]), "");
    }

    #[test]
    fn join_url_trailing_slash_on_base() {
        assert_eq!(
            join_url(&["https://example.com/", "api", "v1"]),
            "https://example.com/api/v1"
        );
    }

    #[test]
    fn join_url_leading_slash_on_segment() {
        assert_eq!(
            join_url(&["https://example.com", "/api", "/v1"]),
            "https://example.com/api/v1"
        );
    }

    #[test]
    fn join_url_extra_slashes() {
        assert_eq!(
            join_url(&["https://example.com/", "/api/", "/v1/"]),
            "https://example.com/api/v1"
        );
    }

    #[test]
    fn join_url_empty_segment_skipped() {
        assert_eq!(
            join_url(&["https://example.com", "", "api"]),
            "https://example.com/api"
        );
    }

    #[test]
    fn trim_trailing_slash_basic() {
        assert_eq!(
            trim_trailing_slash("https://example.com/"),
            "https://example.com"
        );
    }

    #[test]
    fn trim_trailing_slash_multiple() {
        assert_eq!(
            trim_trailing_slash("https://example.com///"),
            "https://example.com"
        );
    }

    #[test]
    fn trim_trailing_slash_no_slash() {
        assert_eq!(
            trim_trailing_slash("https://example.com"),
            "https://example.com"
        );
    }

    #[test]
    fn trim_trailing_slash_just_slash() {
        assert_eq!(trim_trailing_slash("/"), "/");
    }

    #[test]
    fn trim_trailing_slash_scheme_only() {
        assert_eq!(trim_trailing_slash("https://"), "https://");
    }

    #[test]
    fn trim_trailing_slash_path() {
        assert_eq!(
            trim_trailing_slash("https://example.com/path/"),
            "https://example.com/path"
        );
    }

    #[test]
    fn get_domain_https() {
        assert_eq!(
            get_domain("https://example.com/path"),
            Some("example.com".to_owned())
        );
    }

    #[test]
    fn get_domain_http() {
        assert_eq!(
            get_domain("http://example.com"),
            Some("example.com".to_owned())
        );
    }

    #[test]
    fn get_domain_with_port() {
        assert_eq!(
            get_domain("https://example.com:8080/path"),
            Some("example.com".to_owned())
        );
    }

    #[test]
    fn get_domain_no_scheme() {
        assert_eq!(
            get_domain("example.com/path"),
            Some("example.com".to_owned())
        );
    }

    #[test]
    fn get_domain_empty() {
        assert_eq!(get_domain(""), None);
    }

    #[test]
    fn get_domain_just_scheme() {
        assert_eq!(get_domain("https://"), None);
    }

    #[test]
    fn validate_url_valid_https() {
        assert!(validate_url("https://example.com"));
    }

    #[test]
    fn validate_url_valid_http() {
        assert!(validate_url("http://example.com/path"));
    }

    #[test]
    fn validate_url_valid_with_port() {
        assert!(validate_url("https://example.com:8080"));
    }

    #[test]
    fn validate_url_empty() {
        assert!(!validate_url(""));
    }

    #[test]
    fn validate_url_no_scheme() {
        assert!(!validate_url("example.com"));
    }

    #[test]
    fn validate_url_no_host() {
        assert!(!validate_url("https://"));
    }

    #[test]
    fn validate_url_invalid() {
        assert!(!validate_url("not a url at all"));
    }

    #[test]
    fn validate_url_with_path() {
        assert!(validate_url("https://github.com/owner/repo"));
    }
}

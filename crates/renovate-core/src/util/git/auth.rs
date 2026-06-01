use crate::util::host_rules;

pub fn get_git_auth_header(url: &str, host_type: Option<&str>) -> Option<String> {
    let lookup_url = coerce_to_http(url)?;
    let rule = host_rules::find(&host_rules::HostRuleSearch {
        host_type: host_type.map(str::to_owned),
        url: Some(lookup_url),
        read_only: None,
    });
    let token = rule.token.as_deref()?;
    let token = token.strip_prefix("x-access-token:").unwrap_or(token);
    Some(format!("Authorization: Bearer {token}"))
}

pub fn get_git_url_with_auth(url: &str, host_type: Option<&str>) -> String {
    let lookup_url = coerce_to_http(url).unwrap_or_else(|| url.to_owned());
    let rule = host_rules::find(&host_rules::HostRuleSearch {
        host_type: host_type.map(str::to_owned),
        url: Some(lookup_url),
        read_only: None,
    });

    if let Some(token) = rule.token.as_deref() {
        return crate::util::get_http_url(url, Some(token));
    }

    if let (Some(username), Some(password)) = (rule.username.as_deref(), rule.password.as_deref()) {
        let credentials = format!("{}:{}", percent_encode(username), percent_encode(password));
        return crate::util::get_http_url(url, Some(&credentials));
    }

    url.to_owned()
}

fn coerce_to_http(url: &str) -> Option<String> {
    let trimmed = url.trim();
    if trimmed.starts_with("git@")
        || trimmed.starts_with("ssh://")
        || trimmed.starts_with("git://")
        || trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
    {
        Some(crate::util::get_http_url(trimmed, None))
    } else {
        None
    }
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'!'
            | b'~'
            | b'*'
            | b'\''
            | b'('
            | b')' => encoded.push(char::from(byte)),
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_git_auth_header_no_rules_returns_none() {
        assert_eq!(get_git_auth_header("https://example.com/repo", None), None);
    }

    #[test]
    fn get_git_url_with_auth_no_rules_returns_original() {
        assert_eq!(
            get_git_url_with_auth("https://example.com/repo", None),
            "https://example.com/repo"
        );
    }

    #[test]
    fn coerce_to_http_https() {
        assert_eq!(
            coerce_to_http("https://github.com/owner/repo"),
            Some("https://github.com/owner/repo".to_owned())
        );
    }

    #[test]
    fn coerce_to_http_unknown_scheme() {
        assert_eq!(coerce_to_http("ftp://example.com"), None);
    }

    #[test]
    fn percent_encode_alphanumeric() {
        assert_eq!(percent_encode("hello123"), "hello123");
    }

    #[test]
    fn percent_encode_special_chars() {
        assert_eq!(percent_encode("a@b"), "a%40b");
    }

    #[test]
    fn percent_encode_empty() {
        assert_eq!(percent_encode(""), "");
    }
}

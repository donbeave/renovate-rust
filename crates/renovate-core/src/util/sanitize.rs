use regex::Regex;

static CONTROL_CHAR_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"[\x00-\x08\x0b\x0c\x0e-\x1f\x7f]").expect("valid control char regex")
});

pub fn sanitize(input: &str) -> String {
    CONTROL_CHAR_RE.replace_all(input, "").into_owned()
}

pub fn sanitize_url(url: &str) -> String {
    if let Ok(mut parsed) = url::Url::parse(url) {
        if parsed.username().is_empty()
            && parsed.password().is_none()
        {
            return url.to_owned();
        }
        let _ = parsed.set_username("");
        let _ = parsed.set_password(None);
        parsed.to_string()
    } else {
        strip_credentials_from_url(url)
    }
}

fn strip_credentials_from_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(scheme_end) = url.find("://") {
            let after_scheme = scheme_end + 3;
            if at_pos > after_scheme {
                return format!("{}{}", &url[..after_scheme], &url[at_pos + 1..]);
            }
        }
    }
    url.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_removes_control_chars() {
        assert_eq!(sanitize("hello\x00world"), "helloworld");
    }

    #[test]
    fn sanitize_removes_multiple_control_chars() {
        assert_eq!(sanitize("a\x01b\x02c\x03d"), "abcd");
    }

    #[test]
    fn sanitize_preserves_normal_text() {
        assert_eq!(sanitize("hello world"), "hello world");
    }

    #[test]
    fn sanitize_preserves_newlines() {
        assert_eq!(sanitize("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn sanitize_preserves_tabs() {
        assert_eq!(sanitize("hello\tworld"), "hello\tworld");
    }

    #[test]
    fn sanitize_empty() {
        assert_eq!(sanitize(""), "");
    }

    #[test]
    fn sanitize_url_strips_credentials() {
        assert_eq!(
            sanitize_url("https://user:pass@example.com/path"),
            "https://example.com/path"
        );
    }

    #[test]
    fn sanitize_url_strips_user_only() {
        assert_eq!(
            sanitize_url("https://user@example.com/path"),
            "https://example.com/path"
        );
    }

    #[test]
    fn sanitize_url_no_credentials() {
        assert_eq!(
            sanitize_url("https://example.com/path"),
            "https://example.com/path"
        );
    }

    #[test]
    fn sanitize_url_non_parseable() {
        assert_eq!(sanitize_url("not-a-url"), "not-a-url");
    }
}

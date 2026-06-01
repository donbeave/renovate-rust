use crate::string_match::match_regex_or_glob_list;

pub fn match_source_urls(source_url: &str, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return true;
    }
    if source_url.is_empty() {
        return false;
    }
    match_regex_or_glob_list(source_url, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_patterns_matches_all() {
        assert!(match_source_urls("https://github.com/owner/repo", &[]));
    }

    #[test]
    fn exact_match() {
        assert!(match_source_urls(
            "https://github.com/owner/repo",
            &["https://github.com/owner/repo".to_owned()]
        ));
    }

    #[test]
    fn no_match() {
        assert!(!match_source_urls(
            "https://github.com/other/repo",
            &["https://github.com/owner/repo".to_owned()]
        ));
    }

    #[test]
    fn empty_url_no_match() {
        assert!(!match_source_urls(
            "",
            &["https://github.com/owner/repo".to_owned()]
        ));
    }
}

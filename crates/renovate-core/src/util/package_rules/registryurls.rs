use crate::string_match::match_regex_or_glob_list;

pub fn match_registry_urls(registry_urls: &[&str], patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return true;
    }
    registry_urls
        .iter()
        .any(|url| match_regex_or_glob_list(url, patterns))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_patterns_matches_all() {
        assert!(match_registry_urls(&["https://registry.npmjs.org"], &[]));
    }

    #[test]
    fn exact_match() {
        assert!(match_registry_urls(
            &["https://registry.npmjs.org"],
            &["https://registry.npmjs.org".to_owned()]
        ));
    }

    #[test]
    fn any_url_matches() {
        assert!(match_registry_urls(
            &["https://registry.npmjs.org", "https://custom.registry.com"],
            &["https://custom.registry.com".to_owned()]
        ));
    }

    #[test]
    fn no_match() {
        assert!(!match_registry_urls(
            &["https://registry.npmjs.org"],
            &["https://custom.registry.com".to_owned()]
        ));
    }

    #[test]
    fn empty_urls_no_match() {
        assert!(!match_registry_urls(&[], &["https://registry.npmjs.org".to_owned()]));
    }
}

use crate::string_match::match_regex_or_glob_list;

pub fn matches_dep_names(dep_name: &str, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return true;
    }
    match_regex_or_glob_list(dep_name, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_patterns_matches_all() {
        assert!(matches_dep_names("lodash", &[]));
    }

    #[test]
    fn exact_match() {
        assert!(matches_dep_names("lodash", &[("lodash".to_owned())]));
    }

    #[test]
    fn no_match() {
        assert!(!matches_dep_names("lodash", &[("express".to_owned())]));
    }

    #[test]
    fn glob_match() {
        assert!(matches_dep_names("@types/node", &["@types/*".to_owned()]));
    }

    #[test]
    fn negation_excludes() {
        assert!(!matches_dep_names("lodash", &["!lodash".to_owned()]));
    }
}

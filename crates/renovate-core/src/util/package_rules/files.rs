use crate::string_match::match_regex_or_glob_list;

pub fn match_files(file_path: &str, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return true;
    }
    match_regex_or_glob_list(file_path, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_patterns_matches_all() {
        assert!(match_files("package.json", &[]));
    }

    #[test]
    fn exact_match() {
        assert!(match_files("package.json", &["package.json".to_owned()]));
    }

    #[test]
    fn glob_match() {
        assert!(match_files("src/package.json", &["**/package.json".to_owned()]));
    }

    #[test]
    fn no_match() {
        assert!(!match_files("Cargo.toml", &["package.json".to_owned()]));
    }
}

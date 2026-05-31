use crate::string_match::match_regex_or_glob;

pub fn match_current_value(current_value: &str, pattern: Option<&str>) -> bool {
    let Some(pattern) = pattern else {
        return true;
    };
    if pattern.is_empty() {
        return true;
    }
    if current_value.is_empty() {
        return false;
    }
    match_regex_or_glob(current_value, pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_pattern_matches_all() {
        assert!(match_current_value("1.0.0", None));
    }

    #[test]
    fn empty_pattern_matches_all() {
        assert!(match_current_value("1.0.0", Some("")));
    }

    #[test]
    fn exact_match() {
        assert!(match_current_value("1.0.0", Some("1.0.0")));
    }

    #[test]
    fn no_match() {
        assert!(!match_current_value("1.0.0", Some("2.0.0")));
    }

    #[test]
    fn empty_current_value_no_match() {
        assert!(!match_current_value("", Some("1.0.0")));
    }
}

//! Regex utilities — mirrors `lib/util/regex.ts`.

/// Convert a Renovate config regex pattern to a standard Rust regex.
///
/// Handles:
/// - Stripping surrounding `/` delimiters with optional flags (`/pattern/i`)
/// - Wrapping in `^...$` for full-match semantics when delimiters are absent
pub fn config_regex_to_rust_regex(pattern: &str) -> String {
    let trimmed = pattern.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if let (Some('/'), Some('/')) = (trimmed.chars().next(), trimmed.chars().last())
        && trimmed.len() >= 2
    {
        let inner = &trimmed[1..trimmed.len() - 1];
        if inner.is_empty() {
            return String::new();
        }
        return inner.to_owned();
    }

    if let (Some('/'), rest) = (trimmed.chars().next(), &trimmed[1..])
        && let Some(slash_pos) = rest.rfind('/')
    {
        let inner = &rest[..slash_pos];
        if inner.is_empty() {
            return String::new();
        }
        return inner.to_owned();
    }

    if !trimmed.starts_with('^') && !trimmed.ends_with('$') {
        format!("^{trimmed}$")
    } else {
        trimmed.to_owned()
    }
}

/// Validate whether `pattern` is a compilable regex.
pub fn is_valid_regex(pattern: &str) -> bool {
    regex::Regex::new(pattern).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_regex_slash_delimiters() {
        assert_eq!(config_regex_to_rust_regex("/^foo bar$/"), "^foo bar$");
    }

    #[test]
    fn config_regex_slash_delimiters_with_flags() {
        assert_eq!(config_regex_to_rust_regex("/^foo bar$/i"), "^foo bar$");
    }

    #[test]
    fn config_regex_plain_pattern_wraps() {
        assert_eq!(config_regex_to_rust_regex("foo bar"), "^foo bar$");
    }

    #[test]
    fn config_regex_already_anchored() {
        assert_eq!(config_regex_to_rust_regex("^foo bar$"), "^foo bar$");
    }

    #[test]
    fn config_regex_start_anchored_only() {
        assert_eq!(config_regex_to_rust_regex("^foo"), "^foo");
    }

    #[test]
    fn config_regex_end_anchored_only() {
        assert_eq!(config_regex_to_rust_regex("bar$"), "bar$");
    }

    #[test]
    fn config_regex_empty_string() {
        assert_eq!(config_regex_to_rust_regex(""), "");
    }

    #[test]
    fn config_regex_only_slashes() {
        assert_eq!(config_regex_to_rust_regex("//"), "");
    }

    #[test]
    fn config_regex_whitespace_trimmed() {
        assert_eq!(config_regex_to_rust_regex("  foo  "), "^foo$");
    }

    #[test]
    fn is_valid_regex_true() {
        assert!(is_valid_regex("^foo bar$"));
    }

    #[test]
    fn is_valid_regex_false() {
        assert!(!is_valid_regex("[invalid"));
    }

    #[test]
    fn is_valid_regex_empty() {
        assert!(is_valid_regex(""));
    }

    #[test]
    fn is_valid_regex_complex() {
        assert!(is_valid_regex(r"^(?<version>\d+\.\d+\.\d+)$"));
    }

    #[test]
    fn is_valid_regex_unbalanced_parens() {
        assert!(!is_valid_regex("(unclosed"));
    }

    #[test]
    fn config_regex_complex_with_delimiters() {
        assert_eq!(
            config_regex_to_rust_regex("/^(?<version>.+)$/"),
            "^(?<version>.+)$"
        );
    }
}

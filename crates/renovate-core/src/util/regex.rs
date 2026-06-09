//! Regex utilities — mirrors `lib/util/regex.ts`.

use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::{LazyLock, Mutex};

use regex::Regex;

/// Status of the active regex engine as exposed by `regexEngineStatus`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegExpEngineStatus {
    /// RE2-like engine selected.
    Available,
    /// RE2-like engine disabled via `RENOVATE_X_IGNORE_RE2`.
    Ignored,
    /// RE2-like engine bootstrap failed.
    Unavailable {
        /// Error from the unavailable engine.
        err: String,
    },
}

const REGEX_ENGINE_IGNORE_ENV: &str = "RENOVATE_X_IGNORE_RE2";

/// Mirrors `regexEngineStatus`.
pub static REGEX_ENGINE_STATUS: LazyLock<RegExpEngineStatus> = LazyLock::new(|| {
    if env::var(REGEX_ENGINE_IGNORE_ENV).is_ok() {
        RegExpEngineStatus::Ignored
    } else {
        match Regex::new(".*") {
            Ok(_) => RegExpEngineStatus::Available,
            Err(err) => RegExpEngineStatus::Unavailable {
                err: err.to_string(),
            },
        }
    }
});

/// Cache mirrors `regEx()` call memoization keyed by pattern and flags.
static REGEX_CACHE: LazyLock<Mutex<HashMap<String, Regex>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Carries the same metadata as Renovate's custom validation error object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegExError {
    pub validation_message: String,
    pub validation_source: String,
    pub validation_error: String,
}

impl std::fmt::Display for RegExError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.validation_message)
    }
}

impl std::error::Error for RegExError {}

/// Regex used to escape special characters.
pub static ESCAPE_REG_EXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[.*+\-?^${}()|[\\]\\").expect("valid regex escape pattern"));

/// Regex matching newlines.
pub static NEWLINE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\r?\n").unwrap());

/// Matches hidden and invisible Unicode characters.
pub static HIDDEN_UNICODE_CHARACTERS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"([\u{00A0}\u{1680}\u{2000}-\u{200A}\u{2028}\u{2029}\u{202F}\u{205F}\u{3000}\u{200B}\u{200C}\u{FEFF}\u{200E}\u{200F}\u{202A}-\u{202E}\u{00AD})",
    )
    .unwrap()
});

/// @parity lib/util/regex.ts full
/// Compile `pattern` and optionally cache the compiled instance.
pub fn reg_ex(pattern: &str, flags: Option<&str>, use_cache: bool) -> Result<Regex, RegExError> {
    let mut can_be_cached = use_cache;
    if can_be_cached && flags.is_some_and(|f| f.contains('g')) {
        can_be_cached = false;
    }

    let cache_key = match flags {
        Some(flags) => format!("{pattern}:{flags}"),
        None => pattern.to_owned(),
    };

    if can_be_cached {
        let cache = REGEX_CACHE.lock().expect("regex cache lock");
        if let Some(result) = cache.get(&cache_key) {
            return Ok(result.clone());
        }
    }

    let regex = compile_reg_ex(pattern, flags).map_err(|message| RegExError {
        validation_message: message,
        validation_source: pattern.to_owned(),
        validation_error: format!("Invalid regular expression (re2): {pattern}"),
    })?;

    if can_be_cached {
        let mut cache = REGEX_CACHE.lock().expect("regex cache lock");
        cache.insert(cache_key, regex.clone());
    }

    Ok(regex)
}

fn compile_reg_ex(pattern: &str, flags: Option<&str>) -> Result<Regex, String> {
    let mut pattern = pattern.to_owned();
    if let Some(flags) = flags {
        for flag in flags.chars() {
            match flag {
                'g' => {}
                'i' => pattern = format!("(?i){pattern}"),
                'm' => pattern = format!("(?m){pattern}"),
                's' => pattern = format!("(?s){pattern}"),
                'u' => pattern = format!("(?u){pattern}"),
                'x' => pattern = format!("(?x){pattern}"),
                invalid => {
                    return Err(format!("invalid regexp flag: {invalid}"));
                }
            }
        }
    }
    Regex::new(&pattern).map_err(|err| err.to_string())
}

/// Escape regexp meta characters in `input`.
pub fn escape_reg_exp(input: &str) -> String {
    ESCAPE_REG_EXP
        .replace_all(input, |captures: &regex::Captures<'_>| {
            let matched = captures.get(0).expect("captured full match");
            format!("\\{}", matched.as_str())
        })
        .into_owned()
}

/// Convert a Renovate config regex pattern to Rust-compatible regex source.
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
    Regex::new(pattern).is_ok()
}

/// Convert a string into a `\\uXXXX` set in stable, de-duplicated order.
pub fn to_unicode_escape(input: &str) -> String {
    let mut seen = HashSet::new();
    let mut output = Vec::new();
    for ch in input.chars() {
        let item = format!("\\u{:04X}", ch as u32);
        if seen.insert(item.clone()) {
            output.push(item);
        }
    }
    output.join("")
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

    #[test]
    fn reg_ex_caches_results() {
        let first = reg_ex("foo", None, true).expect("regex compiles");
        let second = reg_ex("foo", None, true).expect("regex compiles");
        assert_eq!(first.as_str(), second.as_str());
    }

    #[test]
    fn reg_ex_escapes_input() {
        assert_eq!(escape_reg_exp("a+b*c"), r"a\+b\*c");
    }

    #[test]
    fn reg_ex_newline_pattern() {
        assert!(NEWLINE_REGEX.is_match("a\nb"));
        assert!(NEWLINE_REGEX.is_match("a\rb"));
    }

    #[test]
    fn reg_ex_to_unicode_escape() {
        assert_eq!(to_unicode_escape("ab"), r"\u0061\u0062");
        assert_eq!(to_unicode_escape("aa"), r"\u0061");
    }
}

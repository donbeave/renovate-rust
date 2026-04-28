//! Python `setup.py` dependency extractor.
//!
//! Extracts PyPI dependencies from `setup.py` files by scanning for
//! `install_requires`, `extras_require`, and `tests_require` arguments.
//!
//! Renovate reference:
//! - `lib/modules/manager/pip_setup/extract.ts`
//! - Pattern: `/(^|/)setup\.py$/`
//! - Datasource: PyPI
//!
//! ## Supported forms
//!
//! ```python
//! setup(
//!     install_requires=[
//!         'requests>=2.25.0',
//!         'flask==2.0.0',
//!     ],
//!     extras_require={
//!         'dev': ['pytest>=6.0'],
//!     },
//! )
//! ```

use std::sync::LazyLock;

use regex::Regex;

use crate::extractors::pip::PipExtractedDep;

/// Matches a string literal (single or double quoted, no embedded newlines).
static STRING_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?:'([^']+)'|"([^"]+)")"#).unwrap());

/// Matches the start of a `key=[` or `key={` argument in a `setup()` call.
static KEY_START_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:install_requires|tests_require|setup_requires|extras_require)\s*=\s*[\[\{]")
        .unwrap()
});

/// Extract PyPI deps from a `setup.py` file.
///
/// Returns one `PipExtractedDep` per string literal found in any of the
/// supported requirement list arguments.
pub fn extract(content: &str) -> Vec<PipExtractedDep> {
    let mut result = Vec::new();
    let mut req_strings: Vec<String> = Vec::new();

    // Find each key=[ block and collect string literals from it.
    let mut search_offset = 0;

    while let Some(key_match) = KEY_START_RE.find(&content[search_offset..]) {
        let block_start = search_offset + key_match.end() - 1; // position of opening bracket
        search_offset += key_match.start() + key_match.len();

        // Extract the content within the balanced brackets.
        if let Some(block) = extract_balanced(&content[block_start..]) {
            // Collect all string literals from the block.
            for cap in STRING_RE.captures_iter(block) {
                let s = cap.get(1).or_else(|| cap.get(2)).map(|m| m.as_str());
                if let Some(dep_str) = s {
                    let trimmed = dep_str.trim();
                    if !trimmed.is_empty() {
                        req_strings.push(trimmed.to_owned());
                    }
                }
            }
        }
    }

    if req_strings.is_empty() {
        return result;
    }

    // Re-use the pip requirement parser (infallible — PipExtractError has no variants).
    let joined = req_strings.join("\n");
    let deps = crate::extractors::pip::extract(&joined).unwrap_or_default();
    result.extend(deps);

    result
}

/// Extract the content of a balanced `[...]` or `{...}` block starting at
/// `s[0]`. Returns the slice *including* the opening delimiter.
fn extract_balanced(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }

    let (open, close) = match bytes[0] {
        b'[' => (b'[', b']'),
        b'{' => (b'{', b'}'),
        _ => return None,
    };

    let mut depth: i32 = 0;
    let mut in_single = false;
    let mut in_double = false;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'\'' if !in_double => in_single = !in_single,
            b'"' if !in_single => in_double = !in_double,
            _ if in_single || in_double => {}
            c if c == open => depth += 1,
            c if c == close => {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[..=i]);
                }
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_install_requires() {
        let content = r#"
from setuptools import setup
setup(
    name='mypackage',
    install_requires=[
        'requests>=2.25.0',
        'flask==2.0.0',
    ],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "requests"));
        assert!(deps.iter().any(|d| d.name == "flask"));
    }

    #[test]
    fn extracts_tests_require() {
        let content = r#"
setup(
    tests_require=['pytest>=6.0', 'coverage>=5.0'],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "pytest"));
    }

    #[test]
    fn extracts_extras_require() {
        let content = r#"
setup(
    extras_require={
        'dev': ['mypy>=0.9', 'black>=21.0'],
    },
)
"#;
        let deps = extract(content);
        assert!(deps.iter().any(|d| d.name == "mypy"));
        assert!(deps.iter().any(|d| d.name == "black"));
    }

    #[test]
    fn no_requires_returns_empty() {
        assert!(extract("setup(name='foo')").is_empty());
    }

    #[test]
    fn empty_list_returns_empty() {
        assert!(extract("setup(install_requires=[])").is_empty());
    }

    #[test]
    fn double_quoted_strings() {
        let content = r#"setup(install_requires=["requests>=2.25"])"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "requests");
    }
}

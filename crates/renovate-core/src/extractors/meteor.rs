//! Meteor `package.js` `Npm.depends()` extractor.
//!
//! Parses the `Npm.depends({...})` call in Meteor's `package.js` and returns
//! NPM package dependencies.
//!
//! Renovate reference:
//! - `lib/modules/manager/meteor/extract.ts`
//! - Pattern: `/(^|/)package\.js$/`
//! - Datasource: npm
//!
//! ## Supported form
//!
//! ```js
//! Npm.depends({
//!   "lodash": "4.17.21",
//!   "moment": "2.29.4"
//! });
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single NPM dep extracted from a Meteor `Npm.depends()` block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeteorDep {
    pub name: String,
    pub current_value: String,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches the `Npm.depends({...})` block content (DOTALL).
static NPM_DEPENDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)Npm\.depends\(\{(.*?)\}\)").unwrap());

/// Matches `"name": "version"` or `'name': 'version'` pairs.
static PAIR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"['"]?([\w@/.-]+)['"]?\s*:\s*['"]([^'"]+)['"]"#).unwrap());

/// Extract NPM deps from a Meteor `package.js` file.
pub fn extract(content: &str) -> Vec<MeteorDep> {
    let Some(cap) = NPM_DEPENDS.captures(content) else {
        return Vec::new();
    };
    let block = &cap[1];
    PAIR_RE
        .captures_iter(block)
        .map(|c| MeteorDep {
            name: c[1].to_owned(),
            current_value: c[2].to_owned(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
Package.describe({
  name: 'my-package',
  version: '0.0.1',
});

Npm.depends({
  "lodash": "4.17.21",
  'moment': '2.29.4',
  "semver": "7.5.4"
});
"#;

    // Ported: "returns results" — meteor/extract.spec.ts line 13
    #[test]
    fn extracts_deps() {
        let deps = extract(SAMPLE);
        let lodash = deps.iter().find(|d| d.name == "lodash").unwrap();
        assert_eq!(lodash.current_value, "4.17.21");
        let moment = deps.iter().find(|d| d.name == "moment").unwrap();
        assert_eq!(moment.current_value, "2.29.4");
        assert_eq!(deps.len(), 3);
    }

    // Ported: "returns empty if fails to parse" — meteor/extract.spec.ts line 8
    #[test]
    fn no_npm_depends_returns_empty() {
        assert!(extract("Package.describe({ name: 'foo' });").is_empty());
    }

    // Ported: "returns empty if fails to parse" — meteor/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}

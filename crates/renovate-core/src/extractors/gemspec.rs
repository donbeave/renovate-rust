//! Ruby `.gemspec` dependency extractor.
//!
//! Parses `.gemspec` files and returns gem dependencies (runtime and
//! development) for RubyGems version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/bundler/extract.ts` — handles gemspec deps inline
//! - Pattern: `/(^|/)[^/]*\\.gemspec$/`
//!
//! ## Supported declarations
//!
//! | Form | Treatment |
//! |---|---|
//! | `spec.add_dependency 'rails', '~> 7.0'` | Actionable |
//! | `spec.add_runtime_dependency 'pg'` | No-version skip |
//! | `spec.add_development_dependency 'rspec', '~> 3.0'` | Actionable |
//! | `spec.add_dependency 'rake', git: '...'` | GitSource skip |
//! | `spec.add_dependency 'mylib', path: '...'` | PathSource skip |
//!
//! The receiver prefix (e.g. `spec.`, `gem.`, `s.`) is optional and ignored.

use std::sync::LazyLock;

use regex::Regex;

/// Why a gemspec dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GemspecSkipReason {
    /// No version constraint was specified.
    NoVersion,
    /// Declared with a `git:` or `github:` option.
    GitSource,
    /// Declared with a `path:` option.
    PathSource,
}

/// A single extracted gemspec dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GemspecDep {
    /// Gem name (e.g. `rails`).
    pub name: String,
    /// Joined version constraint (e.g. `~> 7.0, < 8.0`). Empty = unconstrained.
    pub current_value: String,
    /// Whether this is a development dependency.
    pub is_dev: bool,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<GemspecSkipReason>,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches `[prefix.]add[_runtime|_development]_dependency 'name'[, args...]`.
/// Captures: (1) method variant ("", "runtime", "development"), (2) gem name, (3) rest of line.
static ADD_DEP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?i)^\s*(?:\w+\.)?add(?:_(runtime|development))?_dependency\s+['"]([^'"]+)['"](.*)"#,
    )
    .unwrap()
});

/// Captures a quoted version constraint (e.g. `"~> 7.0"`).
static QUOTED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"['"]([^'"]+)['"]"#).unwrap());

/// Extract dependencies from a `.gemspec` file.
pub fn extract(content: &str) -> Vec<GemspecDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        // Strip inline comment.
        let line = raw.split('#').next().unwrap_or("").trim_end();

        let Some(cap) = ADD_DEP.captures(line) else {
            continue;
        };

        let variant = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let name = cap[2].to_owned();
        let rest = &cap[3];

        let is_dev = variant.eq_ignore_ascii_case("development");

        // Detect git / path sources.
        if rest.contains("git:") || rest.contains("github:") {
            out.push(GemspecDep {
                name,
                current_value: String::new(),
                is_dev,
                skip_reason: Some(GemspecSkipReason::GitSource),
            });
            continue;
        }
        if rest.contains("path:") {
            out.push(GemspecDep {
                name,
                current_value: String::new(),
                is_dev,
                skip_reason: Some(GemspecSkipReason::PathSource),
            });
            continue;
        }

        // Collect all quoted version strings from rest.
        let constraints: Vec<&str> = QUOTED
            .captures_iter(rest)
            .map(|c| c.get(1).unwrap().as_str())
            .collect();

        if constraints.is_empty() {
            out.push(GemspecDep {
                name,
                current_value: String::new(),
                is_dev,
                skip_reason: Some(GemspecSkipReason::NoVersion),
            });
        } else {
            out.push(GemspecDep {
                name,
                current_value: constraints.join(", "),
                is_dev,
                skip_reason: None,
            });
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name    = "mygem"
  spec.version = "1.0.0"
  spec.authors = ["Author"]

  spec.add_dependency "rails", ">= 6.0", "< 8.0"
  spec.add_runtime_dependency "sinatra", "~> 2.0"
  spec.add_development_dependency "rspec", "~> 3.12"
  spec.add_development_dependency "rubocop", ">= 1.50"
  spec.add_dependency "rake"
  spec.add_dependency "mylib", path: "../mylib"
  spec.add_dependency "external", git: "https://github.com/org/external"
end
"#;

    #[test]
    fn extracts_runtime_deps() {
        let deps = extract(SAMPLE);
        let rails = deps.iter().find(|d| d.name == "rails").unwrap();
        assert_eq!(rails.current_value, ">= 6.0, < 8.0");
        assert!(!rails.is_dev);
        assert!(rails.skip_reason.is_none());

        let sinatra = deps.iter().find(|d| d.name == "sinatra").unwrap();
        assert_eq!(sinatra.current_value, "~> 2.0");
        assert!(!sinatra.is_dev);
    }

    #[test]
    fn extracts_dev_deps() {
        let deps = extract(SAMPLE);
        let rspec = deps.iter().find(|d| d.name == "rspec").unwrap();
        assert_eq!(rspec.current_value, "~> 3.12");
        assert!(rspec.is_dev);
        assert!(rspec.skip_reason.is_none());

        let rubocop = deps.iter().find(|d| d.name == "rubocop").unwrap();
        assert_eq!(rubocop.current_value, ">= 1.50");
        assert!(rubocop.is_dev);
    }

    #[test]
    fn no_version_gets_skip_reason() {
        let deps = extract(SAMPLE);
        let rake = deps.iter().find(|d| d.name == "rake").unwrap();
        assert_eq!(rake.skip_reason, Some(GemspecSkipReason::NoVersion));
        assert!(rake.current_value.is_empty());
    }

    #[test]
    fn path_source_skipped() {
        let deps = extract(SAMPLE);
        let mylib = deps.iter().find(|d| d.name == "mylib").unwrap();
        assert_eq!(mylib.skip_reason, Some(GemspecSkipReason::PathSource));
    }

    #[test]
    fn git_source_skipped() {
        let deps = extract(SAMPLE);
        let ext = deps.iter().find(|d| d.name == "external").unwrap();
        assert_eq!(ext.skip_reason, Some(GemspecSkipReason::GitSource));
    }

    #[test]
    fn multi_constraint_joined() {
        let content = "spec.add_dependency 'pg', '>= 0.18', '< 2.0'\n";
        let deps = extract(content);
        assert_eq!(deps[0].current_value, ">= 0.18, < 2.0");
    }

    #[test]
    fn alternative_prefix_works() {
        let content = "s.add_runtime_dependency 'nokogiri', '~> 1.15'\n";
        let deps = extract(content);
        assert_eq!(deps[0].name, "nokogiri");
        assert_eq!(deps[0].current_value, "~> 1.15");
    }

    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }
}

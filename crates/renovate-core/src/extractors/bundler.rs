//! Ruby Bundler `Gemfile` dependency extractor.
//!
//! Parses `Gemfile` content line by line with a regex-based scanner and
//! returns gem dependencies ready for RubyGems version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/bundler/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/bundler/index.ts`    — `defaultConfig`
//!
//! ## Supported declarations
//!
//! | Form | Treatment |
//! |---|---|
//! | `gem 'rails', '~> 7.0'` | Actionable — version `~> 7.0` |
//! | `gem 'devise'` | Actionable — unconstrained |
//! | `gem 'pg', '>= 0.18', '< 2.0'` | Actionable — joined as `>= 0.18, < 2.0` |
//! | `gem 'nokogiri', git: '...'` | Skipped — `GitSource` |
//! | `gem 'myapp', path: '...'` | Skipped — `PathSource` |
//! | `gem 'rails', github: 'rails/rails'` | Skipped — `GitSource` |
//! | `ruby '3.2.0'` | Skipped — not a gem |
//!
//! ## Group handling
//!
//! Lines inside `group :development, :test do … end` blocks are marked
//! as [`BundlerDepType::Dev`]. Nested groups and multi-group lines are
//! supported.

use std::sync::LazyLock;

use regex::Regex;

/// Which Gemfile section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundlerDepType {
    /// Top-level or production gem.
    Regular,
    /// Inside a `group :development` or `group :test` block.
    Dev,
}

/// Why a gem is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BundlerSkipReason {
    /// Declared with `git:` or `github:` option.
    GitSource,
    /// Declared with `path:` option.
    PathSource,
}

/// A single extracted gem dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundlerExtractedDep {
    /// Gem name (e.g. `rails`).
    pub name: String,
    /// Version constraint string (e.g. `~> 7.0.0`). Empty = unconstrained.
    pub current_value: String,
    /// Which section this dep came from.
    pub dep_type: BundlerDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<BundlerSkipReason>,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// Matches a `gem 'name'` or `gem "name"` line, capturing the name and the
/// optional trailing arguments (everything after the closing quote).
/// RE2 (the `regex` crate) does not support backreferences, so we accept any
/// closing quote — gem names never contain quote characters, so this is safe.
static GEM_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*gem\s+['"]([^'"]+)['"](.*)"#).unwrap());

/// Extracts a quoted string value from the arguments tail.
static QUOTED_VALUE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"['"]([^'"]+)['"]"#).unwrap());

/// Detects `group :foo, :bar do` or `group 'foo' do`.
static GROUP_START: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*group\s+(.+?)\s+do\s*(?:#.*)?$").unwrap());

/// Detects a bare `end` that closes a block.
static BLOCK_END: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*end\s*(?:#.*)?$").unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Gemfile` and extract all gem dependencies.
pub fn extract(content: &str) -> Vec<BundlerExtractedDep> {
    let mut deps = Vec::new();
    // Depth counter for block nesting; >0 means we're inside at least one block.
    let mut block_depth: usize = 0;
    // Whether the current block (at depth 1) is a dev group.
    let mut in_dev_group = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Track block opens — check for `group` before generic `do` blocks.
        if let Some(cap) = GROUP_START.captures(trimmed) {
            block_depth += 1;
            if block_depth == 1 {
                // Inspect the group names to classify dev vs regular.
                let group_args = &cap[1];
                in_dev_group = is_dev_group(group_args);
            }
            continue;
        }

        // Generic `do` block (non-group): just increment depth.
        if trimmed.ends_with(" do") || trimmed.ends_with("\tdo") || trimmed == "do" {
            block_depth += 1;
            continue;
        }

        // `end` closes the innermost block.
        if BLOCK_END.is_match(trimmed) {
            if block_depth > 0 {
                block_depth -= 1;
                if block_depth == 0 {
                    in_dev_group = false;
                }
            }
            continue;
        }

        let dep_type = if in_dev_group && block_depth == 1 {
            BundlerDepType::Dev
        } else {
            BundlerDepType::Regular
        };

        if let Some(dep) = parse_gem_line(trimmed, dep_type) {
            deps.push(dep);
        }
    }

    deps
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returns true if the group argument list contains `:development` or `:test`.
fn is_dev_group(args: &str) -> bool {
    args.contains("development") || args.contains("test")
}

/// Parse a `gem 'name' [, options…]` line.
fn parse_gem_line(line: &str, dep_type: BundlerDepType) -> Option<BundlerExtractedDep> {
    let cap = GEM_LINE.captures(line)?;
    let name = cap[1].to_owned();
    let tail = cap[2].trim(); // everything after the closing quote of the name

    // Detect git/path source options — skip these gems.
    if tail.contains("git:") || tail.contains("github:") || tail.contains("gitlab:") {
        return Some(BundlerExtractedDep {
            name,
            current_value: String::new(),
            dep_type,
            skip_reason: Some(BundlerSkipReason::GitSource),
        });
    }
    if tail.contains("path:") {
        return Some(BundlerExtractedDep {
            name,
            current_value: String::new(),
            dep_type,
            skip_reason: Some(BundlerSkipReason::PathSource),
        });
    }

    // Collect all quoted version constraints from tail, stopping at the first
    // keyword option (e.g. `require:`, `platforms:`).
    let current_value = collect_version_constraints(tail);

    Some(BundlerExtractedDep {
        name,
        current_value,
        dep_type,
        skip_reason: None,
    })
}

/// Extract version constraint strings from the tail of a gem line.
///
/// Handles:
/// - No version: `` → `""`
/// - Single: `, '~> 7.0'` → `"~> 7.0"`
/// - Multiple: `, '>= 0.18', '< 2.0'` → `">= 0.18, < 2.0"`
fn collect_version_constraints(tail: &str) -> String {
    let mut constraints: Vec<String> = Vec::new();

    for cap in QUOTED_VALUE.captures_iter(tail) {
        let val = cap[1].trim().to_owned();
        // Stop when we hit a keyword-like value (no version operator chars).
        // Version constraints contain digits or operators.
        let looks_like_version = val.chars().any(|c| c.is_ascii_digit())
            || val.starts_with('~')
            || val.starts_with('>')
            || val.starts_with('<')
            || val.starts_with('=')
            || val.starts_with('!');
        if !looks_like_version {
            break;
        }
        constraints.push(val);
    }

    constraints.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn regular(deps: &[BundlerExtractedDep]) -> Vec<&BundlerExtractedDep> {
        deps.iter()
            .filter(|d| d.dep_type == BundlerDepType::Regular)
            .collect()
    }

    fn dev(deps: &[BundlerExtractedDep]) -> Vec<&BundlerExtractedDep> {
        deps.iter()
            .filter(|d| d.dep_type == BundlerDepType::Dev)
            .collect()
    }

    #[test]
    fn simple_gem_with_version() {
        let content = "gem 'rails', '~> 7.0.0'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rails");
        assert_eq!(deps[0].current_value, "~> 7.0.0");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn gem_no_version() {
        let content = "gem 'devise'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "devise");
        assert!(deps[0].current_value.is_empty());
    }

    #[test]
    fn multi_version_constraint() {
        let content = "gem 'pg', '>= 0.18', '< 2.0'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, ">= 0.18, < 2.0");
    }

    #[test]
    fn git_gem_skipped() {
        let content = "gem 'nokogiri', git: 'https://github.com/sparklemotion/nokogiri.git'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(BundlerSkipReason::GitSource));
    }

    #[test]
    fn github_gem_skipped() {
        let content = "gem 'rails', github: 'rails/rails'\n";
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(BundlerSkipReason::GitSource));
    }

    #[test]
    fn path_gem_skipped() {
        let content = "gem 'myapp', path: '../myapp'\n";
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(BundlerSkipReason::PathSource));
    }

    #[test]
    fn group_block_dev_deps() {
        let content = r#"
gem 'rails', '~> 7.0'
gem 'pg', '>= 0.18'

group :development, :test do
  gem 'rspec-rails'
  gem 'byebug'
end
"#;
        let deps = extract(content);
        let reg = regular(&deps);
        let devs = dev(&deps);

        assert_eq!(reg.len(), 2);
        assert_eq!(devs.len(), 2);
        assert!(devs.iter().any(|d| d.name == "rspec-rails"));
        assert!(devs.iter().any(|d| d.name == "byebug"));
    }

    #[test]
    fn development_only_group() {
        let content = r#"
group :development do
  gem 'rubocop', '~> 1.0'
  gem 'pry'
end
"#;
        let deps = extract(content);
        let devs = dev(&deps);
        assert_eq!(devs.len(), 2);
        let rubocop = devs.iter().find(|d| d.name == "rubocop").unwrap();
        assert_eq!(rubocop.current_value, "~> 1.0");
    }

    #[test]
    fn source_and_ruby_lines_ignored() {
        let content = r#"
source 'https://rubygems.org'
ruby '3.2.0'
gem 'rails', '~> 7.0'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rails");
    }

    #[test]
    fn comments_skipped() {
        let content = r#"
# This is a comment
gem 'rails' # inline comment
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rails");
    }

    #[test]
    fn double_quoted_gems() {
        let content = r#"gem "rails", "~> 7.0""#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rails");
        assert_eq!(deps[0].current_value, "~> 7.0");
    }

    #[test]
    fn real_world_gemfile() {
        let content = r#"
source 'https://rubygems.org'
ruby '3.2.0'

gem 'rails', '~> 7.0.4'
gem 'pg', '>= 0.18', '< 2.0'
gem 'puma', '~> 5.0'
gem 'devise', '~> 4.9'
gem 'nokogiri', git: 'https://github.com/sparklemotion/nokogiri.git'
gem 'local_gem', path: '../local_gem'

group :development, :test do
  gem 'rspec-rails', '~> 6.0'
  gem 'byebug'
  gem 'factory_bot_rails'
end

group :development do
  gem 'rubocop', '~> 1.0'
  gem 'rubocop-rails', require: false
end
"#;
        let deps = extract(content);
        let reg = regular(&deps);
        let devs = dev(&deps);

        // rails, pg, puma, devise, nokogiri(skip), local_gem(skip) = 6 regular
        assert_eq!(reg.len(), 6);
        // rspec-rails, byebug, factory_bot_rails, rubocop, rubocop-rails = 5 dev
        assert_eq!(devs.len(), 5);

        let rails = reg.iter().find(|d| d.name == "rails").unwrap();
        assert_eq!(rails.current_value, "~> 7.0.4");
        assert!(rails.skip_reason.is_none());

        let pg = reg.iter().find(|d| d.name == "pg").unwrap();
        assert_eq!(pg.current_value, ">= 0.18, < 2.0");

        let nokogiri = reg.iter().find(|d| d.name == "nokogiri").unwrap();
        assert_eq!(nokogiri.skip_reason, Some(BundlerSkipReason::GitSource));

        let rspec = devs.iter().find(|d| d.name == "rspec-rails").unwrap();
        assert_eq!(rspec.current_value, "~> 6.0");
    }

    #[test]
    fn empty_gemfile_returns_empty() {
        let content = "source 'https://rubygems.org'\n";
        assert!(extract(content).is_empty());
    }
}

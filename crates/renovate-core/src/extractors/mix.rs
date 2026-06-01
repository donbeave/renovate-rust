//! Elixir Mix `mix.exs` dependency extractor.
//!
//! Scans the `deps/0` function body for dependency tuples and returns Hex.pm
//! package dependencies ready for version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/mix/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/mix/index.ts`   — pattern `/(^|/)mix\\.exs$/`
//!
//! ## Supported dependency forms
//!
//! | Form | Treatment |
//! |---|---|
//! | `{:phoenix, "~> 1.7"}` | Actionable — Hex.pm lookup |
//! | `{:plug, ">= 0.0.0", only: :test}` | Actionable — `only:` stripped |
//! | `{:plug, git: "https://..."}` | Skipped — `GitSource` |
//! | `{:plug, github: "org/repo"}` | Skipped — `GitSource` |
//! | `{:plug, path: "../plug"}` | Skipped — `LocalPath` |
//! | `{:plug}` | Skipped — `NoVersion` (no constraint) |

use std::{collections::BTreeMap, sync::LazyLock};

use regex::Regex;

/// Why a Mix dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixSkipReason {
    /// Declared with `git:` or `github:` option.
    GitSource,
    /// Declared with `path:` option.
    LocalPath,
    /// Tuple has no version constraint.
    NoVersion,
}

/// A single extracted Mix dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixExtractedDep {
    /// Package name (e.g. `phoenix`).
    pub name: String,
    /// Version constraint (e.g. `~> 1.7.0`). Empty when `skip_reason` is set.
    pub current_value: String,
    /// Version pinned in the sibling `mix.lock`, when available.
    pub locked_version: Option<String>,
    /// Set when no Hex.pm lookup should be performed.
    pub skip_reason: Option<MixSkipReason>,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// Matches the start of a `deps do … end` or `defp deps do … end` function.
static DEPS_DO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:defp|def)\s+deps\s*(?:\(\s*\))?\s*do").unwrap());

/// Matches a dependency tuple: `{:name, "version"?, options?}`.
///
/// Groups:
/// - 1: package name (atom without colon)
/// - 2: optional version string (contents between quotes)
/// - 3: optional options tail (everything after the version, inside the braces)
static DEP_TUPLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\{:(\w+)(?:\s*,\s*"([^"]+)")?(?:\s*,\s*([^}]+))?\}"#).unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `mix.exs` file and extract all Hex.pm dependencies.
pub fn extract(content: &str) -> Vec<MixExtractedDep> {
    extract_with_lock(content, None)
}

pub fn extract_with_lock(content: &str, lock_content: Option<&str>) -> Vec<MixExtractedDep> {
    // Find the deps function body.
    let Some(deps_block) = extract_deps_block(content) else {
        return Vec::new();
    };

    let locked_versions = lock_content.map(parse_mix_lock).unwrap_or_default();
    let mut deps = Vec::new();
    for cap in DEP_TUPLE.captures_iter(&deps_block) {
        let name = cap[1].to_owned();
        let version = cap.get(2).map(|m| m.as_str().to_owned());
        let opts = cap.get(3).map(|m| m.as_str());
        let locked_version = locked_versions.get(&name).cloned();

        // Check for special source options.
        if let Some(tail) = opts {
            if tail.contains("git:") || tail.contains("github:") {
                deps.push(MixExtractedDep {
                    name,
                    current_value: String::new(),
                    locked_version,
                    skip_reason: Some(MixSkipReason::GitSource),
                });
                continue;
            }
            if tail.contains("path:") {
                deps.push(MixExtractedDep {
                    name,
                    current_value: String::new(),
                    locked_version: None,
                    skip_reason: Some(MixSkipReason::LocalPath),
                });
                continue;
            }
        }

        match version {
            None => {
                deps.push(MixExtractedDep {
                    name,
                    current_value: String::new(),
                    locked_version: None,
                    skip_reason: Some(MixSkipReason::NoVersion),
                });
            }
            Some(v) => {
                deps.push(MixExtractedDep {
                    name,
                    current_value: v,
                    locked_version,
                    skip_reason: None,
                });
            }
        }
    }

    deps
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract the content of the `deps do … end` block.
///
/// Uses a simple brace/do-end depth counter rather than a full Elixir parser.
/// Returns `None` when no deps function is found.
fn extract_deps_block(content: &str) -> Option<String> {
    let start_match = DEPS_DO.find(content)?;
    let after_do = &content[start_match.end()..];

    // Track nesting depth: `do`/`[`/`{` open, `end`/`]`/`}` close.
    // We want everything up to the matching `end` for the deps function.
    let mut depth: i32 = 1;
    let mut result = String::new();

    let mut chars = after_do.chars().peekable();
    // We walk word-by-word for `do`/`end` and char-by-char for `[`, `]`, etc.
    // Since Elixir uses `do…end` for function bodies and `[…]` for list literals,
    // we track both.
    let mut word_buf = String::new();

    while let Some(c) = chars.next() {
        match c {
            '[' | '{' => {
                depth += 1;
                result.push(c);
                word_buf.clear();
            }
            ']' | '}' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                result.push(c);
                word_buf.clear();
            }
            '#' => {
                // Skip line comment.
                result.push(c);
                for c2 in chars.by_ref() {
                    result.push(c2);
                    if c2 == '\n' {
                        break;
                    }
                }
                word_buf.clear();
            }
            c if c.is_alphanumeric() || c == '_' => {
                word_buf.push(c);
                result.push(c);
            }
            _ => {
                // Check for `do` and `end` keywords when we finish a word.
                match word_buf.as_str() {
                    "do" => depth += 1,
                    "end" => {
                        depth -= 1;
                        if depth == 0 {
                            // Remove the trailing "end" from result.
                            let trim_len = result.len().saturating_sub(3);
                            result.truncate(trim_len);
                            break;
                        }
                    }
                    _ => {}
                }
                word_buf.clear();
                result.push(c);
            }
        }
    }

    Some(result)
}

fn parse_mix_lock(content: &str) -> BTreeMap<String, String> {
    static HEX_LOCK: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#""([^"]+)":\s*\{:hex,\s*:[^,]+,\s*"([^"]+)""#).unwrap());
    static GIT_TAG_LOCK: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#""([^"]+)":\s*\{:git,[^}]+\[tag:\s*"([^"]+)""#).unwrap());

    let mut out = BTreeMap::new();
    for cap in HEX_LOCK.captures_iter(content) {
        out.insert(cap[1].to_owned(), cap[2].to_owned());
    }
    for cap in GIT_TAG_LOCK.captures_iter(content) {
        out.insert(cap[1].to_owned(), cap[2].to_owned());
    }
    out
}

/// Return `true` when `value` is a complex range (multiple clauses).
///
/// Matches "and", "or", "||" separators as used by Mix/Hex version constraints.
fn is_complex_mix_range(value: &str) -> bool {
    value.contains(" and ") || value.contains(" or ") || value.contains("||")
}

/// Determine the effective Mix range strategy.
///
/// Mirrors `lib/modules/manager/mix/range.ts` `getRangeStrategy()`.
pub fn get_range_strategy<'a>(range_strategy: &'a str, current_value: Option<&str>) -> &'a str {
    let is_complex = current_value.is_some_and(is_complex_mix_range);
    if range_strategy == "bump" && is_complex {
        return "widen";
    }
    if range_strategy != "auto" {
        return range_strategy;
    }
    if is_complex {
        return "widen";
    }
    "update-lockfile"
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts all dependencies when no lockfile" — manager/mix/extract.spec.ts line 16
    #[test]
    fn simple_hex_dep() {
        let content = r#"
defp deps do
  [
    {:phoenix, "~> 1.7.0"}
  ]
end
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "phoenix");
        assert_eq!(deps[0].current_value, "~> 1.7.0");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts all dependencies when no lockfile" — manager/mix/extract.spec.ts line 16
    #[test]
    fn dep_with_only_option() {
        let content = r#"
defp deps do
  [
    {:ex_doc, "~> 0.27", only: :dev, runtime: false}
  ]
end
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "ex_doc");
        assert_eq!(deps[0].current_value, "~> 0.27");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts all dependencies when no lockfile" — manager/mix/extract.spec.ts line 16
    #[test]
    fn git_dep_skipped() {
        let content = r#"
defp deps do
  [
    {:plug, git: "https://github.com/elixir-plug/plug.git"}
  ]
end
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(MixSkipReason::GitSource));
    }

    // Ported: "extracts all dependencies when no lockfile" — manager/mix/extract.spec.ts line 16
    #[test]
    fn github_dep_skipped() {
        let content = r#"
defp deps do
  [
    {:my_lib, github: "myuser/my_lib"}
  ]
end
"#;
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(MixSkipReason::GitSource));
    }

    // Ported: "extracts all dependencies when no lockfile" — manager/mix/extract.spec.ts line 16
    #[test]
    fn path_dep_skipped() {
        let content = r#"
defp deps do
  [
    {:my_app, path: "../my_app"}
  ]
end
"#;
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(MixSkipReason::LocalPath));
    }

    // Ported: "extracts all dependencies when no lockfile" — manager/mix/extract.spec.ts line 16
    #[test]
    fn dep_without_version_skipped() {
        let content = r#"
defp deps do
  [
    {:jason}
  ]
end
"#;
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(MixSkipReason::NoVersion));
    }

    // Ported: "extracts all dependencies when no lockfile" — manager/mix/extract.spec.ts line 16
    #[test]
    fn real_world_mix_exs() {
        let content = r#"
defmodule MyApp.MixProject do
  use Mix.Project

  def project do
    [app: :my_app, version: "0.1.0"]
  end

  defp deps do
    [
      {:phoenix, "~> 1.7.0"},
      {:phoenix_ecto, "~> 4.4"},
      {:ecto_sql, "~> 3.10"},
      {:postgrex, ">= 0.0.0"},
      {:jason, "~> 1.2"},
      {:plug_cowboy, "~> 2.5"},
      {:ex_doc, "~> 0.27", only: :dev, runtime: false},
      {:git_dep, git: "https://github.com/example/git_dep.git"},
      {:local_dep, path: "../local_dep"},
    ]
  end
end
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 9);

        let phoenix = deps.iter().find(|d| d.name == "phoenix").unwrap();
        assert_eq!(phoenix.current_value, "~> 1.7.0");
        assert!(phoenix.skip_reason.is_none());

        let postgrex = deps.iter().find(|d| d.name == "postgrex").unwrap();
        assert_eq!(postgrex.current_value, ">= 0.0.0");
        assert!(postgrex.skip_reason.is_none());

        let ex_doc = deps.iter().find(|d| d.name == "ex_doc").unwrap();
        assert_eq!(ex_doc.current_value, "~> 0.27");
        assert!(ex_doc.skip_reason.is_none());

        let git_dep = deps.iter().find(|d| d.name == "git_dep").unwrap();
        assert_eq!(git_dep.skip_reason, Some(MixSkipReason::GitSource));

        let local = deps.iter().find(|d| d.name == "local_dep").unwrap();
        assert_eq!(local.skip_reason, Some(MixSkipReason::LocalPath));
    }

    // Ported: "extracts all dependencies and adds the locked version if lockfile present" — mix/extract.spec.ts line 139
    #[test]
    fn applies_locked_versions_from_mix_lock() {
        let content = r#"
defp deps do
  [
    {:postgrex, "~> 0.8.1"},
    {:ranch, "<1.7.0 or ~>1.7.1"},
    {:cowboy, "0.6.0", github: "ninenines/cowboy"},
    {:phoenix, "main", git: "https://github.com/phoenixframework/phoenix.git"},
    {:gun, "~> 2.0.0"}
  ]
end
"#;
        let lock = r#"%{
  "cowboy": {:git, "https://github.com/ninenines/cowboy.git", "0c2e222", [tag: "0.6.0"]},
  "gun": {:hex, :grpc_gun, "2.0.1", "221b792", [:rebar3], [], "hexpm", "hash"},
  "phoenix": {:git, "https://github.com/phoenixframework/phoenix.git", "61cbfeb", [branch: "main"]},
  "postgrex": {:hex, :postgrex, "0.8.4", "344dbb", [:mix], [], "hexpm", "hash"},
  "ranch": {:hex, :ranch, "1.7.1", "6b1fab", [:rebar3], [], "hexpm", "hash"}
}"#;
        let deps = extract_with_lock(content, Some(lock));

        let postgrex = deps.iter().find(|d| d.name == "postgrex").unwrap();
        assert_eq!(postgrex.locked_version.as_deref(), Some("0.8.4"));

        let ranch = deps.iter().find(|d| d.name == "ranch").unwrap();
        assert_eq!(ranch.locked_version.as_deref(), Some("1.7.1"));

        let gun = deps.iter().find(|d| d.name == "gun").unwrap();
        assert_eq!(gun.locked_version.as_deref(), Some("2.0.1"));

        let cowboy = deps.iter().find(|d| d.name == "cowboy").unwrap();
        assert_eq!(cowboy.locked_version.as_deref(), Some("0.6.0"));

        let phoenix = deps.iter().find(|d| d.name == "phoenix").unwrap();
        assert_eq!(phoenix.locked_version, None);
    }

    // Ported: "returns empty for invalid dependency file" — manager/mix/extract.spec.ts line 11
    #[test]
    fn no_deps_function_returns_empty() {
        let content = "defmodule MyApp do\n  def hello, do: :world\nend\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "returns empty for invalid dependency file" — manager/mix/extract.spec.ts line 11
    #[test]
    fn deps_without_do_end_block() {
        let content = r#"
def deps do
  [{:cowboy, "~> 2.0"}]
end
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "cowboy");
    }

    // Ported: "returns same if not auto" — modules/manager/mix/range.spec.ts line 5
    #[test]
    fn mix_range_returns_same_if_not_auto() {
        assert_eq!(get_range_strategy("pin", None), "pin");
        assert_eq!(get_range_strategy("widen", None), "widen");
    }

    // Ported: "widens complex bump" — modules/manager/mix/range.spec.ts line 13
    #[test]
    fn mix_range_widens_complex_bump() {
        let result = get_range_strategy("bump", Some(">= 1.6.0 and < 2.0.0"));
        assert_eq!(result, "widen");
    }

    // Ported: "bumps non-complex bump" — modules/manager/mix/range.spec.ts line 22
    #[test]
    fn mix_range_bumps_non_complex() {
        let result = get_range_strategy("bump", Some("~>1.0.0"));
        assert_eq!(result, "bump");
    }

    // Ported: "widens complex auto" — modules/manager/mix/range.spec.ts line 31
    #[test]
    fn mix_range_widens_complex_auto() {
        let result = get_range_strategy("auto", Some("<1.7.0 or ~>1.7.1"));
        assert_eq!(result, "widen");
    }

    // Ported: "defaults to update-lockfile" — modules/manager/mix/range.spec.ts line 40
    #[test]
    fn mix_range_defaults_to_update_lockfile() {
        let result = get_range_strategy("auto", None);
        assert_eq!(result, "update-lockfile");
    }
}

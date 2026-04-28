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

use std::sync::LazyLock;

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
    // Find the deps function body.
    let Some(deps_block) = extract_deps_block(content) else {
        return Vec::new();
    };

    let mut deps = Vec::new();
    for cap in DEP_TUPLE.captures_iter(&deps_block) {
        let name = cap[1].to_owned();
        let version = cap.get(2).map(|m| m.as_str().to_owned());
        let opts = cap.get(3).map(|m| m.as_str());

        // Check for special source options.
        if let Some(tail) = opts {
            if tail.contains("git:") || tail.contains("github:") {
                deps.push(MixExtractedDep {
                    name,
                    current_value: String::new(),
                    skip_reason: Some(MixSkipReason::GitSource),
                });
                continue;
            }
            if tail.contains("path:") {
                deps.push(MixExtractedDep {
                    name,
                    current_value: String::new(),
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
                    skip_reason: Some(MixSkipReason::NoVersion),
                });
            }
            Some(v) => {
                deps.push(MixExtractedDep {
                    name,
                    current_value: v,
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn no_deps_function_returns_empty() {
        let content = "defmodule MyApp do\n  def hello, do: :world\nend\n";
        assert!(extract(content).is_empty());
    }

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
}

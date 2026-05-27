//! Leiningen `project.clj` dependency extractor.
//!
//! Parses Clojure EDN dep vectors `[group/artifact "version"]` from the
//! standard `:dependencies`, `:managed-dependencies`, `:plugins`,
//! `:pom-plugins`, and `:coords` sections.
//!
//! Renovate reference:
//! - `lib/modules/manager/leiningen/extract.ts`
//! - Pattern: `/(^|/)project\\.clj$/`
//! - Datasource: Clojure (Maven Central + Clojars)
//!
//! ## Dep name expansion
//!
//! | Clojure symbol | Maven coordinates |
//! |---|---|
//! | `org.clojure/clojure` | `org.clojure:clojure` |
//! | `ring` (no slash) | `ring:ring` |
//!
//! ## Not supported (deferred)
//!
//! - `~varName` version references (def-level variables)
//! - Custom `:repositories` registry URLs

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

/// Dep type matching the `:key` section it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeinDepType {
    Dependencies,
    ManagedDependencies,
    Plugins,
    PomPlugins,
    Coords,
}

impl LeinDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            LeinDepType::Dependencies => "dependencies",
            LeinDepType::ManagedDependencies => "managed-dependencies",
            LeinDepType::Plugins => "plugins",
            LeinDepType::PomPlugins => "pom-plugins",
            LeinDepType::Coords => "coords",
        }
    }
}

/// A single extracted Leiningen dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeinDep {
    /// Maven `group:artifact` coordinates (e.g. `"org.clojure:clojure"`).
    pub dep_name: String,
    /// Version string (e.g. `"1.11.1"`).
    pub current_value: String,
    pub dep_type: LeinDepType,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches a dep vector entry: `[symbol "version" ...]`
///
/// Captures: (1) symbol name, (2) version string
static DEP_VECTOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\[\s*([\w.+\-*/!?#$%&|<>=:']+(?:/[\w.+\-*/!?#$%&|<>=:']+)?)\s+"(([^"]|\\")+)""#)
        .unwrap()
});

/// Matches a section keyword at the start of a line (EDN keyword form).
static SECTION_KEY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r":(dependencies|managed-dependencies|plugins|pom-plugins|coords)").unwrap()
});

/// Extracted dep from `extract_from_vectors` — mirrors TypeScript `PackageDependency`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeinExtractedDep {
    pub dep_name: String,
    pub current_value: String,
    pub shared_variable_name: Option<String>,
}

/// Mirrors TypeScript `trimAtKey`.
///
/// Finds `:kw_name` (followed by whitespace) in `s` and returns the slice
/// starting at the first non-whitespace character after the keyword.
pub fn trim_at_key<'a>(s: &'a str, kw_name: &str) -> Option<&'a str> {
    let keyword = format!(":{kw_name}");
    let mut search_from = 0;
    while search_from < s.len() {
        let Some(rel) = s[search_from..].find(&keyword) else {
            return None;
        };
        let abs = search_from + rel;
        let after = abs + keyword.len();
        if s[after..].starts_with(|c: char| c.is_ascii_whitespace()) {
            let rest = &s[after..];
            let value_start = rest.find(|c: char| !c.is_ascii_whitespace())?;
            return Some(&rest[value_start..]);
        }
        search_from = abs + 1;
    }
    None
}

static DEF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^[\s,]*\([\s,]*def[\s,]+",
        r"(?P<varName>[-+*=<>.!?#$%&_|a-zA-Z][-+*=<>.!?#$%&_|a-zA-Z0-9']+)",
        r#"[\s,]*"(?P<stringValue>[^"]*)"[\s,]*\)[\s,]*$"#,
    ))
    .unwrap()
});

/// Mirrors TypeScript `extractVariables`.
pub fn extract_variables(content: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in content.lines() {
        if let Some(caps) = DEF_RE.captures(line) {
            let k = caps.name("varName").unwrap().as_str().to_owned();
            let v = caps.name("stringValue").unwrap().as_str().to_owned();
            result.insert(k, v);
        }
    }
    result
}

/// Mirrors TypeScript `extractFromVectors`.
///
/// `dimensions=2` parses nested `[[dep "version"] ...]`.
/// `dimensions=1` parses a flat `[dep "version"]`.
pub fn extract_from_vectors(
    s: &str,
    vars: &HashMap<String, String>,
    dimensions: u8,
) -> Vec<LeinExtractedDep> {
    if !s.starts_with('[') {
        return Vec::new();
    }
    let dim = dimensions as i32;
    let chars: Vec<char> = s.chars().collect();
    let mut balance: i32 = 0;
    let mut result = Vec::new();
    let mut idx = 0;
    let mut vec_pos: u32 = 0;
    let mut artifact_id = String::new();
    let mut version = String::new();
    let mut comment_level: Option<i32> = None;
    let mut prev_char = '['; // irrelevant before first `[` at depth `dim`

    while idx < chars.len() {
        let ch = chars[idx];

        if idx + 2 < chars.len() && ch == '#' && chars[idx + 1] == '_' && chars[idx + 2] == '[' {
            comment_level = Some(balance);
        }

        if ch == '[' {
            balance += 1;
            if balance == dim {
                vec_pos = 0;
            }
        } else if ch == ']' {
            balance -= 1;

            if comment_level == Some(balance) {
                artifact_id.clear();
                version.clear();
                comment_level = None;
            }

            if balance == dim - 1 {
                if comment_level.is_none() && !artifact_id.is_empty() && !version.is_empty() {
                    let name = artifact_id.trim_matches('"');
                    let dep_name = expand_dep_name(name);
                    if let Some(var_name) = version.strip_prefix('~') {
                        let var_name = var_name.trim_start();
                        if let Some(current_value) = vars.get(var_name) {
                            result.push(LeinExtractedDep {
                                dep_name,
                                current_value: current_value.clone(),
                                shared_variable_name: Some(var_name.to_owned()),
                            });
                        }
                    } else {
                        result.push(LeinExtractedDep {
                            dep_name,
                            current_value: version.trim_matches('"').to_owned(),
                            shared_variable_name: None,
                        });
                    }
                }
                artifact_id.clear();
                version.clear();
            }

            if balance == 0 {
                break;
            }
        } else if balance == dim {
            let is_sp = matches!(ch, ' ' | '\t' | '\n' | '\r' | ',');
            let prev_sp = matches!(prev_char, ' ' | '\t' | '\n' | '\r' | ',');
            if is_sp {
                if !prev_sp {
                    vec_pos += 1;
                }
            } else if vec_pos == 0 {
                artifact_id.push(ch);
            } else if vec_pos == 1 {
                version.push(ch);
            }
        }

        prev_char = ch;
        idx += 1;
    }

    result
}

/// Expands a Clojure dep symbol to Maven `group:artifact` form.
fn expand_dep_name(symbol: &str) -> String {
    if symbol.contains('/') {
        symbol.replace('/', ":")
    } else {
        format!("{symbol}:{symbol}")
    }
}

/// Strip single-line Clojure comments (`;; ...` or `; ...`).
fn strip_comments(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            let mut in_str = false;
            let mut prev = ' ';
            let mut end = line.len();
            for (i, ch) in line.char_indices() {
                if ch == '"' && prev != '\\' {
                    in_str = !in_str;
                }
                if ch == ';' && !in_str {
                    end = i;
                    break;
                }
                prev = ch;
            }
            &line[..end]
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Extract Leiningen dependencies from a `project.clj` file.
pub fn extract(content: &str) -> Vec<LeinDep> {
    let clean = strip_comments(content);
    let mut out = Vec::new();

    // Find each recognized section keyword and scan the vector that follows it.
    for cap in SECTION_KEY.captures_iter(&clean) {
        let key = &cap[1];
        let dep_type = match key {
            "dependencies" => LeinDepType::Dependencies,
            "managed-dependencies" => LeinDepType::ManagedDependencies,
            "plugins" => LeinDepType::Plugins,
            "pom-plugins" => LeinDepType::PomPlugins,
            "coords" => LeinDepType::Coords,
            _ => continue,
        };

        // Grab text from the match position to end of file; the vector
        // for this section starts somewhere after the keyword.
        let start = cap.get(0).unwrap().end();
        let rest = &clean[start..];

        // Find the opening `[` for this section.
        let Some(vec_start) = rest.find('[') else {
            continue;
        };

        // Extract everything up to the balancing `]`.
        let section_text = balanced_brackets(&rest[vec_start..]);

        // Scan all `[symbol "version"]` pairs within the section.
        for dep_cap in DEP_VECTOR.captures_iter(section_text) {
            let symbol = &dep_cap[1];
            let version = &dep_cap[2];
            out.push(LeinDep {
                dep_name: expand_dep_name(symbol),
                current_value: version.to_owned(),
                dep_type: dep_type.clone(),
            });
        }
    }

    out
}

/// Return the slice from `s` (which starts with `[`) up to and including the
/// matching `]`, ignoring brackets inside strings.
fn balanced_brackets(s: &str) -> &str {
    let mut depth: i32 = 0;
    let mut in_str = false;
    let mut prev = ' ';
    for (i, ch) in s.char_indices() {
        if ch == '"' && prev != '\\' {
            in_str = !in_str;
        }
        if !in_str {
            match ch {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        return &s[..=i];
                    }
                }
                _ => {}
            }
        }
        prev = ch;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
(defproject my-project "0.1.0-SNAPSHOT"
  :description "A sample project"
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [ring/ring-core "1.9.6"]
                 [compojure "1.6.3"]]
  :managed-dependencies [[cheshire "5.12.0"]]
  :plugins [[lein-ring "0.12.5"]]
  :profiles {:dev {:dependencies [[ring/ring-mock "0.4.0"]]}})
"#;

    // Ported: "extractPackageFile" — manager/leiningen/extract.spec.ts line 74
    #[test]
    fn extracts_dependencies() {
        let deps = extract(SAMPLE);
        let clojure = deps
            .iter()
            .find(|d| d.dep_name == "org.clojure:clojure")
            .unwrap();
        assert_eq!(clojure.current_value, "1.11.1");
        assert_eq!(clojure.dep_type, LeinDepType::Dependencies);

        let ring = deps
            .iter()
            .find(|d| d.dep_name == "ring:ring-core")
            .unwrap();
        assert_eq!(ring.current_value, "1.9.6");

        // Bare artifact: `compojure` → `compojure:compojure`
        let compojure = deps
            .iter()
            .find(|d| d.dep_name == "compojure:compojure")
            .unwrap();
        assert_eq!(compojure.current_value, "1.6.3");
    }

    // Ported: "extractPackageFile" — manager/leiningen/extract.spec.ts line 74
    #[test]
    fn extracts_managed_dependencies() {
        let deps = extract(SAMPLE);
        let cheshire = deps
            .iter()
            .find(|d| d.dep_name == "cheshire:cheshire")
            .unwrap();
        assert_eq!(cheshire.current_value, "5.12.0");
        assert_eq!(cheshire.dep_type, LeinDepType::ManagedDependencies);
    }

    // Ported: "extractPackageFile" — manager/leiningen/extract.spec.ts line 74
    #[test]
    fn extracts_plugins() {
        let deps = extract(SAMPLE);
        let plugin = deps
            .iter()
            .find(|d| d.dep_name == "lein-ring:lein-ring")
            .unwrap();
        assert_eq!(plugin.current_value, "0.12.5");
        assert_eq!(plugin.dep_type, LeinDepType::Plugins);
    }

    // Ported: "extractPackageFile" — manager/leiningen/extract.spec.ts line 74
    #[test]
    fn dev_profile_dependencies_also_extracted() {
        let deps = extract(SAMPLE);
        let mock = deps.iter().find(|d| d.dep_name == "ring:ring-mock");
        assert!(mock.is_some());
        assert_eq!(mock.unwrap().current_value, "0.4.0");
    }

    #[test]
    fn expand_dep_name_slash_form() {
        assert_eq!(
            expand_dep_name("org.clojure/clojure"),
            "org.clojure:clojure"
        );
    }

    #[test]
    fn expand_dep_name_bare_form() {
        assert_eq!(expand_dep_name("ring"), "ring:ring");
    }

    // Ported: "trimAtKey" — manager/leiningen/extract.spec.ts line 10
    #[test]
    fn trim_at_key_cases() {
        assert_eq!(trim_at_key("foo", "bar"), None);
        assert_eq!(trim_at_key(":dependencies    ", "dependencies"), None);
        assert_eq!(
            trim_at_key(":dependencies \nfoobar", "dependencies"),
            Some("foobar")
        );
        assert_eq!(
            trim_at_key(
                ":parent-project {:coords [my-org/my-parent \"4.3.0\"]\n:inherit [:profiles]}",
                "coords"
            ),
            Some("[my-org/my-parent \"4.3.0\"]\n:inherit [:profiles]}")
        );
    }

    // Ported: "extractFromVectors" — manager/leiningen/extract.spec.ts line 22
    #[test]
    fn extract_from_vectors_cases() {
        let empty: HashMap<String, String> = HashMap::new();

        assert!(extract_from_vectors("", &empty, 2).is_empty());
        assert!(extract_from_vectors("[]", &empty, 2).is_empty());
        assert!(extract_from_vectors("[[]]", &empty, 2).is_empty());
        assert!(extract_from_vectors("[#_[foo/bar \"1.2.3\"]]", &empty, 2).is_empty());

        let res = extract_from_vectors("[[foo/bar \"1.2.3\"]]", &empty, 2);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].dep_name, "foo:bar");
        assert_eq!(res[0].current_value, "1.2.3");
        assert_eq!(res[0].shared_variable_name, None);

        let mut vars = HashMap::new();
        vars.insert("baz".to_owned(), "1.2.3".to_owned());
        let res = extract_from_vectors("[[foo/bar ~baz]]", &vars, 2);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].dep_name, "foo:bar");
        assert_eq!(res[0].current_value, "1.2.3");
        assert_eq!(res[0].shared_variable_name, Some("baz".to_owned()));

        let res = extract_from_vectors(
            "[\t[foo/bar \"1.2.3\"]\n[\"foo/baz\"  \"4.5.6\"] ]",
            &empty,
            2,
        );
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].dep_name, "foo:bar");
        assert_eq!(res[0].current_value, "1.2.3");
        assert_eq!(res[1].dep_name, "foo:baz");
        assert_eq!(res[1].current_value, "4.5.6");

        let res = extract_from_vectors(
            "[my-org/my-parent \"4.3.0\"]\n:inherit [:profiles]}",
            &empty,
            1,
        );
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].dep_name, "my-org:my-parent");
        assert_eq!(res[0].current_value, "4.3.0");
    }

    // Ported: "extractVariables" — manager/leiningen/extract.spec.ts line 239
    #[test]
    fn extract_variables_cases() {
        let res = extract_variables("(def foo \"1\")");
        assert_eq!(res.get("foo"), Some(&"1".to_owned()));

        let res = extract_variables("(def foo\"2\")");
        assert_eq!(res.get("foo"), Some(&"2".to_owned()));

        let res = extract_variables("(def foo \"3\")\n(def bar \"4\")");
        assert_eq!(res.get("foo"), Some(&"3".to_owned()));
        assert_eq!(res.get("bar"), Some(&"4".to_owned()));
    }

    #[test]
    fn empty_project_returns_no_deps() {
        assert!(extract("(defproject foo \"1.0\")").is_empty());
    }

    #[test]
    fn comments_stripped_before_parsing() {
        let content = "(defproject x \"1.0\"\n  :dependencies [[org.clojure/clojure \"1.11.1\"] ; keep\n                  [ring \"1.9.6\"]])";
        let deps = extract(content);
        assert!(deps.iter().any(|d| d.dep_name == "org.clojure:clojure"));
        assert!(deps.iter().any(|d| d.dep_name == "ring:ring"));
    }
}

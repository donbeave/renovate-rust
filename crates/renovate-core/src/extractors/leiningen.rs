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

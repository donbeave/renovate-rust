//! Travis CI `.travis.yml` Node.js version extractor.
//!
//! Extracts Node.js version constraints from `node_js:` lists in
//! Travis CI configuration files. LTS aliases (`lts/*`, `stable`, `node`)
//! are skipped since they don't pin a specific version.
//!
//! Renovate reference:
//! - `lib/modules/manager/travis/extract.ts`
//! - Pattern: `/^\\.travis\\.ya?ml$/`
//!
//! ## Supported form
//!
//! ```yaml
//! language: node_js
//! node_js:
//!   - "18"
//!   - "20.9.0"
//!   - lts/*      # skipped
//! ```

/// A single Node.js version dep extracted from `.travis.yml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TravisDep {
    /// Node.js version string (e.g. `"18"`, `"20.9.0"`).
    pub version: String,
}

/// Parse `.travis.yml` and extract all Node.js version constraints.
pub fn extract(content: &str) -> Vec<TravisDep> {
    let mut out = Vec::new();
    let mut in_node_js = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let indent = leading_spaces(line);

        // Detect `node_js:` key (at any indent level).
        if trimmed == "node_js:" || trimmed.starts_with("node_js:") {
            in_node_js = true;
            // Handle inline single value: `node_js: "18"`
            if let Some(val) = trimmed.strip_prefix("node_js:") {
                let v = val.trim().trim_matches('"').trim_matches('\'');
                if !v.is_empty() {
                    maybe_push(&mut out, v);
                }
            }
            continue;
        }

        if !in_node_js {
            continue;
        }

        // List item at deeper or same indent continues the node_js list.
        if trimmed.starts_with("- ") {
            let val = trimmed
                .strip_prefix("- ")
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .trim_matches('\'');
            maybe_push(&mut out, val);
            continue;
        }

        // A non-list, non-empty key at the root level exits node_js section.
        if indent == 0 && !trimmed.starts_with('-') {
            in_node_js = false;
        }
    }

    out
}

fn maybe_push(out: &mut Vec<TravisDep>, version: &str) {
    // Skip aliases and non-version strings.
    if version.is_empty()
        || version.starts_with("lts/")
        || matches!(version, "stable" | "node" | "iojs" | "latest")
    {
        return;
    }
    out.push(TravisDep {
        version: version.to_owned(),
    });
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns results" — travis/extract.spec.ts line 18
    #[test]
    fn extracts_node_js_versions() {
        let content = r#"
language: node_js
node_js:
  - "18"
  - "20.9.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].version, "18");
        assert_eq!(deps[1].version, "20.9.0");
    }

    // Ported: "returns results" — travis/extract.spec.ts line 18
    #[test]
    fn lts_alias_skipped() {
        let content = "node_js:\n  - lts/*\n  - \"18\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, "18");
    }

    // Ported: "returns results" — travis/extract.spec.ts line 18
    #[test]
    fn stable_skipped() {
        let content = "node_js:\n  - stable\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "returns empty if fails to parse" — travis/extract.spec.ts line 13
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns empty if fails to parse" — travis/extract.spec.ts line 13
    #[test]
    fn no_node_js_key_returns_empty() {
        let content = "language: python\npython:\n  - \"3.11\"\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "returns empty if fails to parse" — travis/extract.spec.ts line 13
    #[test]
    fn invalid_content_returns_empty() {
        assert!(extract("blahhhhh:foo:@what\n").is_empty());
    }

    // Ported: "handles matrix node_js syntax with node_js string" — travis/extract.spec.ts line 29
    #[test]
    fn matrix_jobs_include_node_js_string() {
        let content = "jobs:\n  include:\n    - env: js-tests\n      language: node_js\n      node_js: '11.10.1'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, "11.10.1");
    }

    // Ported: "handles matrix node_js syntax with node_js array 2" — travis/extract.spec.ts line 60
    #[test]
    fn matrix_jobs_include_node_js_multiline_list() {
        let content = "jobs:\n  include:\n    - env: js-tests\n      language: node_js\n      node_js:\n        - '11.10.1'\n        - '11.10.2'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].version, "11.10.1");
        assert_eq!(deps[1].version, "11.10.2");
    }

    // Ported: "handles matrix node_js syntax with alias" — travis/extract.spec.ts line 78
    #[test]
    fn matrix_alias_node_js_string() {
        let content = "matrix:\n  include:\n    - env: js-tests\n      language: node_js\n      node_js: '11.10.1'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, "11.10.1");
    }

    // Ported: "handles invalid matrix node_js syntax" — travis/extract.spec.ts line 91
    #[test]
    fn matrix_without_node_js_returns_empty() {
        let content = "jobs:\n  include:\n    - invalid: '1.0'\n";
        assert!(extract(content).is_empty());
    }
}

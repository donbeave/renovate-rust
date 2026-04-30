//! Go modules (`go.mod`) dependency extractor.
//!
//! Parses `go.mod` files and returns the set of module dependencies with
//! their version strings, ready for Go module proxy lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/gomod/extract.ts`    — `extractPackageFile`
//! - `lib/modules/manager/gomod/line-parser.ts` — `parseLine`
//!
//! ## Parsing rules
//!
//! - `require <module> <version>` — single-line require.
//! - Multi-line `require (…)` blocks — each non-blank line inside is a dep.
//! - `replace X => Y version` — remote replacement; Y+version is extracted as a dep.
//! - `replace X => ../path` — local replacement; deps replaced with a local path are skipped.
//! - `replace (…)` blocks — multi-line replace directives.
//! - `exclude (…)` blocks are ignored entirely.
//! - `// indirect` comment is preserved in the dep record but does not skip.
//!
//! ## Skip reasons
//!
//! | Reason | Description |
//! |---|---|
//! | `PseudoVersion` | Version is a Go pseudo-version (`v0.0.0-TIMESTAMP-HASH`). |
//! | `LocalReplace` | Module is replaced by a local path (not a registry dep). |

use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;

/// Why a go.mod dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoModSkipReason {
    /// Version is a Go pseudo-version (built from a commit timestamp+hash).
    PseudoVersion,
    /// Module is replaced by a local path in a `replace` directive.
    LocalReplace,
}

/// A single extracted go.mod dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoModExtractedDep {
    /// Go module path (e.g. `github.com/gorilla/mux`), or `"go"` for the Go directive.
    pub module_path: String,
    /// Declared version (e.g. `v1.8.1`, `v25.1.0+incompatible`).
    pub current_value: String,
    /// Whether the dep is marked `// indirect`.
    pub is_indirect: bool,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<GoModSkipReason>,
    /// Set for the `go X.Y` directive; `datasource` would be `golang-version`.
    pub is_go_directive: bool,
    /// Set for the `toolchain goX.Y.Z` directive.
    pub is_toolchain_directive: bool,
    /// Set for a `replace X => Y version` directive (remote replacement).
    pub is_replace_directive: bool,
}

// ── Compiled regexes ───────────────────────────────────────────────────────

/// Matches a single-line `require <module> <version>` directive.
static SINGLE_REQUIRE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*require\s+(\S+)\s+(\S+)").unwrap());

/// Matches the start of a `require (` block.
static REQUIRE_BLOCK_START: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*require\s*\(\s*$").unwrap());

/// Matches `module <version>` inside a require block.
static BLOCK_DEP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+(\S+)\s+(\S+)").unwrap());

/// Matches the end of a block (`)`).
static BLOCK_END: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*\)\s*$").unwrap());

/// Matches a `replace <old> => <new>` directive (local path form).
/// Local replacement: `replace X => ../path` (replacement has no version).
static REPLACE_LOCAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*replace\s+(\S+)(?:\s+\S+)?\s*=>\s*(\./|\.\./)").unwrap());

/// Matches single-line `replace X [oldVer] => Y newVer` for remote replacements.
static SINGLE_REPLACE_REMOTE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*replace\s+\S+(?:\s+\S+)?\s*=>\s*(\S+)\s+(\S+)").unwrap());

/// Matches the start of a `replace (` block.
static REPLACE_BLOCK_START: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*replace\s*\(\s*$").unwrap());

/// Matches `X [oldVer] => Y newVer` inside a replace block (indented).
static REPLACE_BLOCK_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+\S+(?:\s+\S+)?\s*=>\s*(\S+)\s+(\S+)").unwrap());

/// Go pseudo-version pattern: `vX.Y.Z-[pre.]YYYYMMDDHHMMSS-abcdefabcdef`.
/// The optional `pre.` prefix appears in pre-release pseudo-versions.
static PSEUDO_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^v\d+\.\d+\.\d+-(?:\d+\.)?\d{14}-[0-9a-f]+$").unwrap());

/// Matches exclude block start.
static EXCLUDE_BLOCK_START: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*exclude\s*\(\s*$").unwrap());

/// Matches `go <version>` directive (e.g. `go 1.21.3` or `go 1.21`).
static GO_DIRECTIVE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*go\s+(\d+\.\d+(?:\.\d+)?)\s*$").unwrap());

/// Matches `toolchain go<version>` directive (e.g. `toolchain go1.23.3`).
static TOOLCHAIN_DIRECTIVE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*toolchain\s+go(\d+\.\d+(?:\.\d+)?)\s*$").unwrap());

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a `go.mod` file and extract all `require` and `replace` directives.
pub fn extract(content: &str) -> Vec<GoModExtractedDep> {
    // First pass: collect locally-replaced module paths.
    let local_replaces: HashSet<String> = collect_local_replaces(content);

    let mut deps = Vec::new();
    let mut in_require_block = false;
    let mut in_exclude_block = false;
    let mut in_replace_block = false;

    for line in content.lines() {
        // Strip inline comments for matching purposes.
        let is_indirect = line.contains("// indirect");
        let bare = strip_comment(line);

        if in_exclude_block {
            if BLOCK_END.is_match(bare) {
                in_exclude_block = false;
            }
            continue;
        }

        if EXCLUDE_BLOCK_START.is_match(bare) {
            in_exclude_block = true;
            continue;
        }

        if in_replace_block {
            if BLOCK_END.is_match(bare) {
                in_replace_block = false;
                continue;
            }
            if let Some(cap) = REPLACE_BLOCK_ITEM.captures(bare) {
                let replacement = cap[1].to_owned();
                let version = cap[2].to_owned();
                if !replacement.starts_with("./") && !replacement.starts_with("../") {
                    deps.push(make_replace_dep(replacement, version, is_indirect));
                }
            }
            continue;
        }

        if REPLACE_BLOCK_START.is_match(bare) {
            in_replace_block = true;
            continue;
        }

        if in_require_block {
            if BLOCK_END.is_match(bare) {
                in_require_block = false;
                continue;
            }
            if let Some(cap) = BLOCK_DEP.captures(bare) {
                let module_path = cap[1].to_owned();
                let current_value = cap[2].to_owned();
                deps.push(make_dep(
                    module_path,
                    current_value,
                    is_indirect,
                    &local_replaces,
                ));
            }
            continue;
        }

        if REQUIRE_BLOCK_START.is_match(bare) {
            in_require_block = true;
            continue;
        }

        if let Some(cap) = GO_DIRECTIVE.captures(bare) {
            deps.push(GoModExtractedDep {
                module_path: "go".to_owned(),
                current_value: cap[1].to_owned(),
                is_indirect: false,
                skip_reason: None,
                is_go_directive: true,
                is_toolchain_directive: false,
                is_replace_directive: false,
            });
            continue;
        }

        if let Some(cap) = TOOLCHAIN_DIRECTIVE.captures(bare) {
            deps.push(GoModExtractedDep {
                module_path: "go".to_owned(),
                current_value: cap[1].to_owned(),
                is_indirect: false,
                skip_reason: None,
                is_go_directive: false,
                is_toolchain_directive: true,
                is_replace_directive: false,
            });
            continue;
        }

        // Single-line remote replace: `replace X [oldVer] => Y newVer`
        if let Some(cap) = SINGLE_REPLACE_REMOTE.captures(bare) {
            let replacement = cap[1].to_owned();
            let version = cap[2].to_owned();
            if !replacement.starts_with("./") && !replacement.starts_with("../") {
                deps.push(make_replace_dep(replacement, version, is_indirect));
                continue;
            }
        }

        if let Some(cap) = SINGLE_REQUIRE.captures(bare) {
            let module_path = cap[1].to_owned();
            let current_value = cap[2].to_owned();
            deps.push(make_dep(
                module_path,
                current_value,
                is_indirect,
                &local_replaces,
            ));
        }
    }

    deps
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn collect_local_replaces(content: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    for line in content.lines() {
        if let Some(cap) = REPLACE_LOCAL.captures(line) {
            set.insert(cap[1].to_owned());
        }
    }
    set
}

fn make_dep(
    module_path: String,
    current_value: String,
    is_indirect: bool,
    local_replaces: &HashSet<String>,
) -> GoModExtractedDep {
    let skip_reason = if local_replaces.contains(&module_path) {
        Some(GoModSkipReason::LocalReplace)
    } else if PSEUDO_VERSION.is_match(&current_value) {
        Some(GoModSkipReason::PseudoVersion)
    } else {
        None
    };

    GoModExtractedDep {
        module_path,
        current_value,
        is_indirect,
        skip_reason,
        is_go_directive: false,
        is_toolchain_directive: false,
        is_replace_directive: false,
    }
}

fn make_replace_dep(
    module_path: String,
    current_value: String,
    is_indirect: bool,
) -> GoModExtractedDep {
    GoModExtractedDep {
        module_path,
        current_value,
        is_indirect,
        skip_reason: None,
        is_go_directive: false,
        is_toolchain_directive: false,
        is_replace_directive: true,
    }
}

fn strip_comment(line: &str) -> &str {
    if let Some(idx) = line.find("//") {
        line[..idx].trim_end()
    } else {
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts single-line requires" — gomod/extract.spec.ts line 16
    #[test]
    fn single_line_require() {
        let content = r#"
module github.com/example/mymod

require github.com/gorilla/mux v1.8.1
require github.com/pkg/errors v0.9.1
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.module_path == "github.com/gorilla/mux" && d.current_value == "v1.8.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.module_path == "github.com/pkg/errors" && d.current_value == "v0.9.1")
        );
    }

    // Ported: "extracts multi-line requires" — gomod/extract.spec.ts line 26
    #[test]
    fn require_block() {
        let content = r#"
require (
    github.com/gorilla/mux v1.8.1
    golang.org/x/net v0.14.0
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.module_path == "github.com/gorilla/mux")
        );
        assert!(deps.iter().any(|d| d.module_path == "golang.org/x/net"));
    }

    // Ported: "extracts multi-line requires" — gomod/extract.spec.ts line 26
    #[test]
    fn indirect_deps_included() {
        let content = "require github.com/davecgh/go-spew v1.0.0 // indirect\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].is_indirect);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "marks placeholder pseudo versions with skipReason invalid-version" — gomod/extract.spec.ts line 426
    #[test]
    fn pseudo_version_skipped() {
        let content = "require github.com/foo/bar v0.0.0-20230901123456-abcdef123456\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GoModSkipReason::PseudoVersion));
    }

    // Ported: "extracts replace directives from multi-line and single line" — gomod/extract.spec.ts line 48
    #[test]
    fn local_replace_skipped() {
        let content = r#"
require github.com/pkg/errors v0.7.0
replace github.com/pkg/errors => ../errors
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GoModSkipReason::LocalReplace));
    }

    // Ported: "ignores exclude directives from multi-line and single line" — gomod/extract.spec.ts line 193
    #[test]
    fn exclude_block_ignored() {
        let content = r#"
require github.com/good/dep v1.0.0

exclude (
    github.com/bad/dep v1.0.0
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].module_path, "github.com/good/dep");
    }

    // Ported: "extracts replace directives from multi-line and single line" — gomod/extract.spec.ts line 48
    #[test]
    fn incompatible_version_included() {
        let content = "require github.com/Azure/sdk v25.1.0+incompatible\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "v25.1.0+incompatible");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts replace directives from multi-line and single line" — gomod/extract.spec.ts line 48
    #[test]
    fn fixture_1() {
        let content = r#"module github.com/renovate-tests/gomod1

require github.com/pkg/errors v0.7.0
require github.com/aws/aws-sdk-go v1.15.21
require github.com/davecgh/go-spew v1.0.0 // indirect
require golang.org/x/foo v1.0.0
require github.com/rarkins/foo abcdef1
require gopkg.in/russross/blackfriday.v1 v1.0.0
require github.com/Azure/azure-sdk-for-go v25.1.0+incompatible

replace github.com/pkg/errors => ../errors
replace golang.org/x/foo => github.com/pravesht/gocql v0.0.0

require github.com/caarlos0/env v3.5.0+incompatible
require sigs.k8s.io/structured-merge-diff/v4 v4.7.0
"#;
        let deps = extract(content);
        // pkg/errors has local replace → skipped
        let errors = deps
            .iter()
            .find(|d| d.module_path == "github.com/pkg/errors")
            .unwrap();
        assert_eq!(errors.skip_reason, Some(GoModSkipReason::LocalReplace));

        // golang.org/x/foo: replaced with a module (not local path) — NOT skipped
        let foo = deps
            .iter()
            .find(|d| d.module_path == "golang.org/x/foo")
            .unwrap();
        assert!(foo.skip_reason.is_none());

        // azure sdk is included (incompatible but not pseudo-version)
        assert!(
            deps.iter()
                .any(|d| d.module_path == "github.com/Azure/azure-sdk-for-go")
        );

        // indirect dep is included
        let spew = deps
            .iter()
            .find(|d| d.module_path == "github.com/davecgh/go-spew")
            .unwrap();
        assert!(spew.is_indirect);
    }

    // Ported: "returns null for empty" — gomod/extract.spec.ts line 12
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts `go` directive %s as a `%goMod` extracted constraint as a SemVer-minor compatible range" — gomod/extract.spec.ts line 528
    #[test]
    fn go_directive_extracted() {
        for version in &["1.19", "1.19.0", "1.19.5"] {
            let content = format!("module github.com/renovate-tests/gomod\ngo {version}\n");
            let deps = extract(&content);
            let go_dep = deps.iter().find(|d| d.module_path == "go").unwrap();
            assert_eq!(go_dep.current_value, *version);
            assert!(go_dep.is_go_directive);
            assert!(go_dep.skip_reason.is_none());
        }
    }

    // Ported: "ignores directives unrelated to dependencies" — gomod/extract.spec.ts line 402
    #[test]
    fn unrelated_directives_ignored() {
        let content = "module github.com/renovate-tests/gomod\n\
            godebug asynctimerchan=0\n\
            godebug (\n  default=go1.21\n  panicnil=1\n)\n\
            retract v3.0.0\n\
            retract [v2.0.0,v2.0.5]\n\
            retract (\n    v1.0.0\n    v1.0.1\n)\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "ignores empty spaces in multi-line requires" — gomod/extract.spec.ts line 34
    #[test]
    fn empty_lines_inside_require_block() {
        let content = "module github.com/renovate-tests/gomod\nrequire (\n\tcloud.google.com/go v0.45.1\n\n\tgithub.com/Microsoft/go-winio v0.4.15-0.20190919025122-fc70bd9a86b5 // indirect\n)\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.module_path == "cloud.google.com/go" && d.current_value == "v0.45.1")
        );
        // Pseudo-version is extracted but marked as skipped
        assert!(
            deps.iter()
                .any(|d| d.module_path == "github.com/Microsoft/go-winio"
                    && d.skip_reason == Some(GoModSkipReason::PseudoVersion))
        );
    }

    // Ported: "extracts the toolchain directive" — gomod/extract.spec.ts line 212
    #[test]
    fn toolchain_directive_extracted() {
        let content = "module github.com/renovate-tests/gomod\ngo 1.23\ntoolchain go1.23.3\n";
        let deps = extract(content);
        let go_dep = deps.iter().find(|d| d.is_go_directive).unwrap();
        assert_eq!(go_dep.current_value, "1.23");

        let toolchain_dep = deps.iter().find(|d| d.is_toolchain_directive).unwrap();
        assert_eq!(toolchain_dep.module_path, "go");
        assert_eq!(toolchain_dep.current_value, "1.23.3");
        assert!(toolchain_dep.skip_reason.is_none());
    }

    // Ported: "extracts replace directives from multi-line and single line" — gomod/extract.spec.ts line 48
    #[test]
    fn replace_directives_multi_line_and_single_line() {
        let content = r#"module github.com/renovate-tests/gomod
go 1.23
replace golang.org/x/foo => github.com/pravesht/gocql v0.0.0
replace (
      k8s.io/client-go => k8s.io/client-go v0.21.9
      )
replace (
  k8s.io/cloud-provider => k8s.io/cloud-provider v0.17.3
  k8s.io/cluster-bootstrap => k8s.io/cluster-bootstrap v0.17.3 // indirect
  k8s.io/code-generator => k8s.io/code-generator v0.17.3
)
"#;
        let deps = extract(content);

        let go = deps.iter().find(|d| d.is_go_directive).unwrap();
        assert_eq!(go.current_value, "1.23");

        let replace_deps: Vec<_> = deps.iter().filter(|d| d.is_replace_directive).collect();
        assert_eq!(replace_deps.len(), 5);

        let gocql = replace_deps
            .iter()
            .find(|d| d.module_path == "github.com/pravesht/gocql")
            .unwrap();
        assert_eq!(gocql.current_value, "v0.0.0");
        assert!(!gocql.is_indirect);

        let client_go = replace_deps
            .iter()
            .find(|d| d.module_path == "k8s.io/client-go")
            .unwrap();
        assert_eq!(client_go.current_value, "v0.21.9");

        let cluster = replace_deps
            .iter()
            .find(|d| d.module_path == "k8s.io/cluster-bootstrap")
            .unwrap();
        assert!(cluster.is_indirect);
        assert_eq!(cluster.current_value, "v0.17.3");
    }

    // Ported: "extracts single-line tool directives" — gomod/extract.spec.ts line 263
    #[test]
    fn tool_directive_single_line_ignored() {
        // `tool X` lines are not package deps; they just reference an already-required module.
        // The extractor should produce the same output as without the tool line.
        let content = "require github.com/oapi-codegen/oapi-codegen/v2 v2.4.1 // indirect\n\
                       tool github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].module_path,
            "github.com/oapi-codegen/oapi-codegen/v2"
        );
        assert_eq!(deps[0].current_value, "v2.4.1");
        assert!(deps[0].is_indirect);
    }

    // Ported: "extracts multi-line tool directives" — gomod/extract.spec.ts line 282
    #[test]
    fn tool_directive_multi_line_ignored() {
        let content = "require github.com/oapi-codegen/oapi-codegen/v2 v2.4.1 // indirect\n\
                       tool (\n\
                         github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen\n\
                       )\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].module_path,
            "github.com/oapi-codegen/oapi-codegen/v2"
        );
    }

    // Ported: "extracts tool directives with required modules" — gomod/extract.spec.ts line 304
    #[test]
    fn tool_directive_with_required_module_not_indirect() {
        let content = "require github.com/oapi-codegen/oapi-codegen/v2 v2.4.1\n\
                       tool github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(!deps[0].is_indirect);
    }

    // Ported: "extracts tool directives with no matching dependencies" — gomod/extract.spec.ts line 389
    #[test]
    fn tool_directive_alone_produces_no_deps() {
        let content = "tool github.com/foo/bar/sub/cmd/hello\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "marks placeholder pseudo versions with skipReason invalid-version" — gomod/extract.spec.ts line 426
    #[test]
    fn placeholder_pseudo_versions_have_skip_reason() {
        let content = r#"module github.com/renovate-tests/gomod
go 1.19
require (
  github.com/foo/bar v1.2.3
  github.com/baz/qux v0.0.0-00010101000000-000000000000
  github.com/example/local v0.0.0-00010101000000-000000000000 // indirect
  github.com/non/placeholder v1.2.4-0.20230101120000-abcdef123456
  monorepo v0.0.0-00010101000000-000000000000
)
"#;
        let deps = extract(content);
        // go directive + 5 requires = 6 total
        assert_eq!(deps.len(), 6);

        let bar = deps
            .iter()
            .find(|d| d.module_path == "github.com/foo/bar")
            .unwrap();
        assert!(bar.skip_reason.is_none());

        let baz = deps
            .iter()
            .find(|d| d.module_path == "github.com/baz/qux")
            .unwrap();
        assert_eq!(baz.skip_reason, Some(GoModSkipReason::PseudoVersion));

        let local = deps
            .iter()
            .find(|d| d.module_path == "github.com/example/local")
            .unwrap();
        assert_eq!(local.skip_reason, Some(GoModSkipReason::PseudoVersion));
        assert!(local.is_indirect);

        let non_placeholder = deps
            .iter()
            .find(|d| d.module_path == "github.com/non/placeholder")
            .unwrap();
        // v1.2.4-0.20230101120000-abcdef123456 is a real pseudo-version (not placeholder)
        assert_eq!(
            non_placeholder.skip_reason,
            Some(GoModSkipReason::PseudoVersion)
        );

        let monorepo = deps.iter().find(|d| d.module_path == "monorepo").unwrap();
        assert_eq!(monorepo.skip_reason, Some(GoModSkipReason::PseudoVersion));
    }

    // Ported: "extracts replace directives from non-public module path" — gomod/extract.spec.ts line 136
    #[test]
    fn replace_directive_non_public_module_path() {
        let content = r#"module github.com/JamieTanna-Mend-testing/tka-9783-golang-pro-main
go 1.25.5
require pro-lib v0.0.0-00010101000000-000000000000
replace pro-lib => github.com/ns-rpro-dev-tests/golang-pro-lib/libs/src/ns v0.0.0-20260219031232-e6910bd8fb97
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);

        let go = deps.iter().find(|d| d.is_go_directive).unwrap();
        assert_eq!(go.current_value, "1.25.5");

        let pro_lib = deps.iter().find(|d| d.module_path == "pro-lib").unwrap();
        assert_eq!(pro_lib.skip_reason, Some(GoModSkipReason::PseudoVersion));

        let replacement = deps.iter().find(|d| d.is_replace_directive).unwrap();
        assert_eq!(
            replacement.module_path,
            "github.com/ns-rpro-dev-tests/golang-pro-lib/libs/src/ns"
        );
        assert_eq!(
            replacement.current_value,
            "v0.0.0-20260219031232-e6910bd8fb97"
        );
        assert!(replacement.skip_reason.is_none());
    }
}

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

// ── parseLine types ───────────────────────────────────────────────────────────

/// Dependency type returned by [`parse_line`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoModLineDepType {
    /// `go X.Y` directive.
    Golang,
    /// `toolchain goX.Y.Z` directive.
    Toolchain,
    /// `require` dependency.
    Require,
    /// `replace` dependency.
    Replace,
    /// `require` or `replace` dependency marked `// indirect`.
    Indirect,
    /// `tool` directive.
    Tool,
}

/// Skip reason returned by [`parse_line`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoModLineSkipReason {
    InvalidVersion,
    UnspecifiedVersion,
    UnversionedReference,
    LocalDependency,
}

/// A single go.mod line parsed by [`parse_line`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoModParsedLine {
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub datasource: &'static str,
    pub dep_name: String,
    pub dep_type: GoModLineDepType,
    pub enabled: Option<bool>,
    pub skip_reason: Option<GoModLineSkipReason>,
    pub versioning: Option<&'static str>,
    pub commit_message_topic: Option<&'static str>,
    pub multi_line: bool,
    pub digest_one_and_only: bool,
}

// ── parseLine regexes ─────────────────────────────────────────────────────────

/// `go <version>` — any non-whitespace version string.
static PL_GO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*go\s+(\S+)\s*$").unwrap());

/// `toolchain go<version>`.
static PL_TOOLCHAIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*toolchain\s+go(\S+)\s*$").unwrap());

/// `[require] <module> <version> [// <comment>]`
static PL_REQUIRE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?P<kw>require)?\s+(?P<module>\S+)\s+(?P<version>\S+)(?:\s*//\s*(?P<comment>\S+)\s*)?$"#,
    )
    .unwrap()
});

/// `[replace] <from> => <to> [<version>] [// <comment>]`
static PL_REPLACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?P<kw>replace)?\s+\S+\s*=>\s*(?P<to>\S+)(?:\s+(?P<version>\S+))?(?:\s*//\s*(?P<comment>\S+)\s*)?$"#,
    )
    .unwrap()
});

/// `[tool] <module>` — no trailing version.
static PL_TOOL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<kw>tool)?\s+(?P<module>\S+)\s*$").unwrap());

/// Pseudo-version digest extractor.
static PL_PSEUDO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"v\d+\.\d+\.\d+-(?:\w+\.)?(?:0\.)?\d{14}-(?P<digest>[a-f0-9]{12})").unwrap()
});

const PLACEHOLDER_PSEUDO_VERSION: &str = "v0.0.0-00010101000000-000000000000";

fn trim_quotes(s: &str) -> &str {
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn is_semver_version(v: &str) -> bool {
    let bare = v.strip_prefix('v').unwrap_or(v);
    semver::Version::parse(bare).is_ok()
}

fn extract_pseudo_digest(v: &str) -> Option<String> {
    PL_PSEUDO
        .captures(v)
        .and_then(|cap| cap.name("digest"))
        .map(|m| m.as_str().to_owned())
}

/// Parse one line from a `go.mod` file.
///
/// Mirrors TypeScript `parseLine` in `lib/modules/manager/gomod/line-parser.ts`.
/// Returns `None` if the line does not match any known directive.
pub fn parse_line(input: &str) -> Option<GoModParsedLine> {
    // go directive
    if let Some(cap) = PL_GO.captures(input) {
        let current_value = cap[1].to_owned();
        let skip_reason = semver::VersionReq::parse(&current_value)
            .is_err()
            .then_some(GoModLineSkipReason::InvalidVersion);
        return Some(GoModParsedLine {
            current_value: Some(current_value),
            current_digest: None,
            datasource: "golang-version",
            dep_name: "go".to_owned(),
            dep_type: GoModLineDepType::Golang,
            enabled: None,
            skip_reason,
            versioning: Some("go-mod-directive"),
            commit_message_topic: Some("go module directive"),
            multi_line: false,
            digest_one_and_only: false,
        });
    }

    // toolchain directive
    if let Some(cap) = PL_TOOLCHAIN.captures(input) {
        let current_value = cap[1].to_owned();
        let skip_reason =
            (!is_semver_version(&current_value)).then_some(GoModLineSkipReason::InvalidVersion);
        return Some(GoModParsedLine {
            current_value: Some(current_value),
            current_digest: None,
            datasource: "golang-version",
            dep_name: "go".to_owned(),
            dep_type: GoModLineDepType::Toolchain,
            enabled: None,
            skip_reason,
            versioning: None,
            commit_message_topic: Some("go toolchain directive"),
            multi_line: false,
            digest_one_and_only: false,
        });
    }

    // require directive
    if let Some(cap) = PL_REQUIRE.captures(input) {
        // must not contain "=>" (that would be a replace)
        if input.contains("=>") {
            // fall through to replace
        } else {
            let keyword = cap.name("kw").map(|m| m.as_str());
            let module = trim_quotes(&cap["module"]).to_owned();
            let current_value = cap["version"].to_owned();
            let comment = cap.name("comment").map(|m| m.as_str());
            let multi_line = keyword.is_none();
            let indirect = comment == Some("indirect");

            let (skip_reason, current_digest, digest_one_and_only, versioning) =
                if is_semver_version(&current_value) {
                    if let Some(digest) = extract_pseudo_digest(&current_value) {
                        let is_placeholder = current_value == PLACEHOLDER_PSEUDO_VERSION;
                        let sr = is_placeholder.then_some(GoModLineSkipReason::InvalidVersion);
                        (sr, Some(digest), true, Some("loose"))
                    } else {
                        (None, None, false, None)
                    }
                } else {
                    (Some(GoModLineSkipReason::InvalidVersion), None, false, None)
                };

            let dep_type = if indirect {
                GoModLineDepType::Indirect
            } else {
                GoModLineDepType::Require
            };

            return Some(GoModParsedLine {
                current_value: Some(current_value),
                current_digest,
                datasource: "go",
                dep_name: module,
                dep_type,
                enabled: indirect.then_some(false),
                skip_reason,
                versioning,
                commit_message_topic: None,
                multi_line,
                digest_one_and_only,
            });
        }
    }

    // replace directive
    if let Some(cap) = PL_REPLACE.captures(input) {
        let keyword = cap.name("kw").map(|m| m.as_str());
        let to_raw = &cap["to"];
        let dep_name = trim_quotes(to_raw).to_owned();
        let current_value_raw = cap.name("version").map(|m| m.as_str().to_owned());
        let comment = cap.name("comment").map(|m| m.as_str());
        let multi_line = keyword.is_none();
        let indirect = comment == Some("indirect");

        // Local path replacement
        if dep_name.starts_with('/') || dep_name.starts_with('.') {
            return Some(GoModParsedLine {
                current_value: None,
                current_digest: None,
                datasource: "go",
                dep_name,
                dep_type: GoModLineDepType::Replace,
                enabled: None,
                skip_reason: Some(GoModLineSkipReason::LocalDependency),
                versioning: None,
                commit_message_topic: None,
                multi_line,
                digest_one_and_only: false,
            });
        }

        let (skip_reason, current_value, current_digest, digest_one_and_only, versioning) =
            match current_value_raw {
                None => (
                    Some(GoModLineSkipReason::UnspecifiedVersion),
                    None,
                    None,
                    false,
                    None,
                ),
                Some(cv) => {
                    if is_semver_version(&cv) {
                        if let Some(digest) = extract_pseudo_digest(&cv) {
                            let is_placeholder = cv == PLACEHOLDER_PSEUDO_VERSION;
                            let sr = is_placeholder.then_some(GoModLineSkipReason::InvalidVersion);
                            (sr, Some(cv), Some(digest), true, Some("loose"))
                        } else {
                            (None, Some(cv), None, false, None)
                        }
                    } else {
                        (
                            Some(GoModLineSkipReason::InvalidVersion),
                            Some(cv),
                            None,
                            false,
                            None,
                        )
                    }
                }
            };

        let dep_type = if indirect {
            GoModLineDepType::Indirect
        } else {
            GoModLineDepType::Replace
        };

        return Some(GoModParsedLine {
            current_value,
            current_digest,
            datasource: "go",
            dep_name,
            dep_type,
            enabled: indirect.then_some(false),
            skip_reason,
            versioning,
            commit_message_topic: None,
            multi_line,
            digest_one_and_only,
        });
    }

    // tool directive
    if let Some(cap) = PL_TOOL.captures(input) {
        let keyword = cap.name("kw").map(|m| m.as_str());
        let module = trim_quotes(&cap["module"]).to_owned();
        let multi_line = keyword.is_none();
        return Some(GoModParsedLine {
            current_value: None,
            current_digest: None,
            datasource: "go",
            dep_name: module,
            dep_type: GoModLineDepType::Tool,
            enabled: None,
            skip_reason: Some(GoModLineSkipReason::UnversionedReference),
            versioning: None,
            commit_message_topic: None,
            multi_line,
            digest_one_and_only: false,
        });
    }

    None
}

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
    /// Set to `Some(false)` for indirect tool-module candidates that do not match a tool directive.
    pub enabled: Option<bool>,
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

/// Matches a single-line `tool <module>` directive.
static TOOL_DIRECTIVE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*tool\s+(\S+)\s*$").unwrap());

/// Matches the start of a `tool (` block.
static TOOL_BLOCK_START: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*tool\s*\(\s*$").unwrap());

/// Matches `module/path` inside a tool block.
static TOOL_BLOCK_ITEM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*(\S+)\s*$").unwrap());

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a `go.mod` file and extract all `require` and `replace` directives.
pub fn extract(content: &str) -> Vec<GoModExtractedDep> {
    // First pass: collect locally-replaced module paths.
    let local_replaces: HashSet<String> = collect_local_replaces(content);
    let tool_directives = collect_tool_directives(content);

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
                enabled: None,
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
                enabled: None,
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

    apply_tool_directives(&mut deps, &tool_directives);
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

fn collect_tool_directives(content: &str) -> Vec<String> {
    let mut tools = Vec::new();
    let mut in_tool_block = false;

    for line in content.lines() {
        let bare = strip_comment(line);

        if in_tool_block {
            if BLOCK_END.is_match(bare) {
                in_tool_block = false;
                continue;
            }
            if let Some(cap) = TOOL_BLOCK_ITEM.captures(bare) {
                tools.push(cap[1].to_owned());
            }
            continue;
        }

        if TOOL_BLOCK_START.is_match(bare) {
            in_tool_block = true;
            continue;
        }

        if let Some(cap) = TOOL_DIRECTIVE.captures(bare) {
            tools.push(cap[1].to_owned());
        }
    }

    tools
}

fn apply_tool_directives(deps: &mut [GoModExtractedDep], tool_directives: &[String]) {
    if tool_directives.is_empty() {
        return;
    }

    let mut active_indirect_modules = HashSet::new();
    for tool in tool_directives {
        if let Some(module_path) = deps
            .iter()
            .filter(|dep| is_require_dep(dep) && module_path_matches_tool(&dep.module_path, tool))
            .max_by_key(|dep| dep.module_path.len())
            .map(|dep| dep.module_path.clone())
        {
            active_indirect_modules.insert(module_path);
        }
    }

    for dep in deps {
        if is_require_dep(dep)
            && dep.is_indirect
            && !active_indirect_modules.contains(&dep.module_path)
        {
            dep.enabled = Some(false);
        }
    }
}

fn is_require_dep(dep: &GoModExtractedDep) -> bool {
    !dep.is_go_directive && !dep.is_toolchain_directive && !dep.is_replace_directive
}

fn module_path_matches_tool(module_path: &str, tool: &str) -> bool {
    tool == module_path
        || tool
            .strip_prefix(module_path)
            .is_some_and(|rest| rest.starts_with('/'))
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
        enabled: None,
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
        enabled: None,
    }
}

fn strip_comment(line: &str) -> &str {
    if let Some(idx) = line.find("//") {
        line[..idx].trim_end()
    } else {
        line
    }
}

// ── artifacts-extra (mirrors lib/modules/manager/gomod/artifacts-extra.ts) ──

/// A single dependency change detected between two go.mod files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraDep {
    pub dep_name: String,
    pub current_value: String,
    pub new_value: String,
}

/// Compare two go.mod file contents and return the dependencies that changed,
/// excluding any in `exclude_deps`.
///
/// Mirrors `getExtraDeps` from `lib/modules/manager/gomod/artifacts-extra.ts`.
pub fn get_extra_deps(before: &str, after: &str, exclude_deps: &[&str]) -> Vec<ExtraDep> {
    use std::collections::HashMap;

    // Build sets of lines for set-difference diff.
    let before_lines: HashSet<&str> = before.lines().collect();
    let after_lines: HashSet<&str> = after.lines().collect();

    // Removed lines = in before but not in after.
    // Added lines = in after but not in before.
    let removed_set: HashSet<&str> = before_lines.difference(&after_lines).copied().collect();
    let added_set: HashSet<&str> = after_lines.difference(&before_lines).copied().collect();

    // Parse removed lines → build map: expanded_dep_name → old_version.
    // Use Vec to maintain before-file line order.
    let mut rm_deps: Vec<(String, String)> = Vec::new();
    let mut rm_seen: HashSet<String> = HashSet::new();
    for line in before.lines() {
        if !removed_set.contains(line) {
            continue;
        }
        let Some(parsed) = parse_line(line) else {
            continue;
        };
        let Some(current_value) = parsed.current_value else {
            continue;
        };
        let expanded = if parsed.dep_type == GoModLineDepType::Toolchain {
            format!("{} (toolchain)", parsed.dep_name)
        } else {
            parsed.dep_name.clone()
        };
        if !rm_seen.contains(&expanded) {
            rm_deps.push((expanded.clone(), current_value));
            rm_seen.insert(expanded);
        }
    }

    // Parse added lines → build map: expanded_dep_name → new_version.
    let mut add_deps: HashMap<String, String> = HashMap::new();
    for line in added_set {
        let Some(parsed) = parse_line(line) else {
            continue;
        };
        let Some(current_value) = parsed.current_value else {
            continue;
        };
        let expanded = if parsed.dep_type == GoModLineDepType::Toolchain {
            format!("{} (toolchain)", parsed.dep_name)
        } else {
            parsed.dep_name.clone()
        };
        add_deps.entry(expanded).or_insert(current_value);
    }

    // Combine: for each removed dep, if there's a matching added dep and it's not excluded,
    // record the change.
    let mut result = Vec::new();
    for (dep_name, old_val) in rm_deps {
        if exclude_deps.contains(&dep_name.as_str()) {
            continue;
        }
        if let Some(new_val) = add_deps.get(&dep_name) {
            result.push(ExtraDep {
                dep_name,
                current_value: old_val,
                new_value: new_val.clone(),
            });
        }
    }
    result
}

/// Generate a markdown table of dependency changes.
///
/// Mirrors `extraDepsTable` from `lib/modules/manager/gomod/artifacts-extra.ts`.
pub fn extra_deps_table(deps: &[ExtraDep]) -> String {
    let headers = ["**Package**", "**Change**"];

    let rows: Vec<[String; 2]> = deps
        .iter()
        .map(|dep| {
            [
                format!("`{}`", dep.dep_name),
                format!("`{}` -> `{}`", dep.current_value, dep.new_value),
            ]
        })
        .collect();

    // Compute column widths.
    let col_widths: [usize; 2] = std::array::from_fn(|col| {
        let header_len = headers[col].len();
        let data_max = rows.iter().map(|row| row[col].len()).max().unwrap_or(0);
        header_len.max(data_max)
    });

    // Format a row: ` cell  ` padded to column width, joined by `|`.
    let fmt_row = |cells: &[&str; 2]| -> String {
        cells
            .iter()
            .zip(&col_widths)
            .map(|(cell, &w)| format!(" {:<w$} ", cell, w = w))
            .collect::<Vec<_>>()
            .join("|")
    };

    let mut lines = Vec::new();
    lines.push(format!("|{}|", fmt_row(&[headers[0], headers[1]])));

    // Separator: `:` + dashes.
    let sep = [
        format!(":{}", "-".repeat(col_widths[0] - 1)),
        format!(":{}", "-".repeat(col_widths[1] - 1)),
    ];
    lines.push(format!(
        "|{}|",
        fmt_row(&[sep[0].as_str(), sep[1].as_str()])
    ));

    for row in &rows {
        lines.push(format!(
            "|{}|",
            fmt_row(&[row[0].as_str(), row[1].as_str()])
        ));
    }

    lines.join("\n")
}

/// Generate a full notice for extra dependency changes in a go.mod file.
///
/// Returns `None` if either file is missing or no extra deps are found.
/// Mirrors `getExtraDepsNotice` from `lib/modules/manager/gomod/artifacts-extra.ts`.
pub fn get_extra_deps_notice(
    go_mod_before: Option<&str>,
    go_mod_after: Option<&str>,
    exclude_deps: &[&str],
) -> Option<String> {
    let before = go_mod_before?;
    let after = go_mod_after?;

    let extra_deps = get_extra_deps(before, after, exclude_deps);
    if extra_deps.is_empty() {
        return None;
    }

    let go_updated = extra_deps.iter().any(|d| d.dep_name == "go");
    let toolchain_updated = extra_deps.iter().any(|d| d.dep_name == "go (toolchain)");
    let other_count = extra_deps.len() - usize::from(go_updated) - usize::from(toolchain_updated);

    let mut lines = Vec::new();
    lines.push(
        "In order to perform the update(s) described in the table above, \
        Renovate ran the `go get` command, which resulted in the following additional change(s):"
            .to_owned(),
    );
    lines.push(String::new());
    lines.push(String::new());

    if other_count == 1 {
        lines.push("- 1 additional dependency was updated".to_owned());
    } else if other_count > 1 {
        lines.push(format!(
            "- {other_count} additional dependencies were updated"
        ));
    }

    if go_updated {
        lines.push("- The `go` directive was updated for compatibility reasons".to_owned());
    }

    lines.push(String::new());
    lines.push(String::new());
    lines.push("Details:".to_owned());
    lines.push(String::new());
    lines.push(String::new());
    lines.push(extra_deps_table(&extra_deps));

    Some(lines.join("\n"))
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
        assert_eq!(deps[0].enabled, None);
    }

    // Ported: "extracts tool directives of sub-modules" — gomod/extract.spec.ts line 323
    #[test]
    fn tool_directive_sub_modules_disable_non_matching_indirects() {
        let content = r#"require (
  github.com/foo/bar v1.2.3
  github.com/foo/bar/sub1/sub2 v4.5.6 // indirect
  github.com/foo/bar/sub1 v7.8.9 // indirect
  github.com/foo/bar/sub1/sub2/cmd/hell v10.11.12 // indirect
)
tool github.com/foo/bar/sub1/sub2/cmd/hello
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 4);

        let root = deps
            .iter()
            .find(|d| d.module_path == "github.com/foo/bar")
            .unwrap();
        assert_eq!(root.current_value, "v1.2.3");
        assert!(!root.is_indirect);
        assert_eq!(root.enabled, None);

        let matched = deps
            .iter()
            .find(|d| d.module_path == "github.com/foo/bar/sub1/sub2")
            .unwrap();
        assert_eq!(matched.current_value, "v4.5.6");
        assert!(matched.is_indirect);
        assert_eq!(matched.enabled, None);

        let shorter = deps
            .iter()
            .find(|d| d.module_path == "github.com/foo/bar/sub1")
            .unwrap();
        assert_eq!(shorter.current_value, "v7.8.9");
        assert!(shorter.is_indirect);
        assert_eq!(shorter.enabled, Some(false));

        let non_boundary = deps
            .iter()
            .find(|d| d.module_path == "github.com/foo/bar/sub1/sub2/cmd/hell")
            .unwrap();
        assert_eq!(non_boundary.current_value, "v10.11.12");
        assert!(non_boundary.is_indirect);
        assert_eq!(non_boundary.enabled, Some(false));
    }

    // Ported: "extracts tool directives with exact match" — gomod/extract.spec.ts line 370
    #[test]
    fn tool_directive_exact_match_keeps_indirect_enabled() {
        let content = "require github.com/foo/bar v1.2.3 // indirect\n\
                       tool github.com/foo/bar\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].module_path, "github.com/foo/bar");
        assert_eq!(deps[0].current_value, "v1.2.3");
        assert!(deps[0].is_indirect);
        assert_eq!(deps[0].enabled, None);
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

    // --- parse_line tests ---

    // Ported: "should return null for invalid input" — gomod/line-parser.spec.ts line 4
    #[test]
    fn parse_line_invalid_returns_none() {
        assert!(parse_line("invalid").is_none());
    }

    // Ported: "should parse go version" — gomod/line-parser.spec.ts line 8
    #[test]
    fn parse_line_go_version() {
        let r = parse_line("go 1.23").unwrap();
        assert_eq!(r.current_value.as_deref(), Some("1.23"));
        assert_eq!(r.datasource, "golang-version");
        assert_eq!(r.dep_name, "go");
        assert_eq!(r.dep_type, GoModLineDepType::Golang);
        assert_eq!(r.versioning, Some("go-mod-directive"));
        assert_eq!(r.commit_message_topic, Some("go module directive"));
        assert!(r.skip_reason.is_none());
    }

    // Ported: "should skip invalid go version" — gomod/line-parser.spec.ts line 21
    #[test]
    fn parse_line_go_version_invalid() {
        let r = parse_line("go invalid").unwrap();
        assert_eq!(r.current_value.as_deref(), Some("invalid"));
        assert_eq!(r.dep_type, GoModLineDepType::Golang);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
    }

    // Ported: "should parse toolchain version" — gomod/line-parser.spec.ts line 35
    #[test]
    fn parse_line_toolchain_version() {
        let r = parse_line("toolchain go1.23").unwrap();
        assert_eq!(r.current_value.as_deref(), Some("1.23"));
        assert_eq!(r.datasource, "golang-version");
        assert_eq!(r.dep_name, "go");
        assert_eq!(r.dep_type, GoModLineDepType::Toolchain);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
        assert_eq!(r.commit_message_topic, Some("go toolchain directive"));
    }

    // Ported: "should skip invalid toolchain version" — gomod/line-parser.spec.ts line 48
    #[test]
    fn parse_line_toolchain_version_invalid() {
        let r = parse_line("toolchain go-invalid").unwrap();
        assert_eq!(r.current_value.as_deref(), Some("-invalid"));
        assert_eq!(r.dep_type, GoModLineDepType::Toolchain);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
    }

    // Ported: "should parse require definition" — gomod/line-parser.spec.ts line 61
    #[test]
    fn parse_line_require_definition() {
        let r = parse_line("require foo/foo v1.2").unwrap();
        assert_eq!(r.current_value.as_deref(), Some("v1.2"));
        assert_eq!(r.datasource, "go");
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.dep_type, GoModLineDepType::Require);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
        assert!(!r.multi_line);
    }

    // Ported: "should parse require definition with pseudo-version" — gomod/line-parser.spec.ts line 73
    #[test]
    fn parse_line_require_pseudo_version() {
        let r = parse_line("require foo/foo v0.0.0-20210101000000-000000000000").unwrap();
        assert_eq!(
            r.current_value.as_deref(),
            Some("v0.0.0-20210101000000-000000000000")
        );
        assert_eq!(r.current_digest.as_deref(), Some("000000000000"));
        assert_eq!(r.dep_type, GoModLineDepType::Require);
        assert!(r.digest_one_and_only);
        assert_eq!(r.versioning, Some("loose"));
        assert!(r.skip_reason.is_none());
    }

    // Ported: "should parse require definition with placeholder pseudo-version" — gomod/line-parser.spec.ts line 87
    #[test]
    fn parse_line_require_placeholder_pseudo_version() {
        let r = parse_line("require foo/foo v0.0.0-00010101000000-000000000000").unwrap();
        assert_eq!(r.current_digest.as_deref(), Some("000000000000"));
        assert_eq!(r.dep_type, GoModLineDepType::Require);
        assert!(r.digest_one_and_only);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
        assert_eq!(r.versioning, Some("loose"));
    }

    // Ported: "should parse require multi-line" — gomod/line-parser.spec.ts line 102
    #[test]
    fn parse_line_require_multiline() {
        let r = parse_line("        foo/foo v1.2").unwrap();
        assert_eq!(r.dep_type, GoModLineDepType::Require);
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
        assert!(r.multi_line);
    }

    // Ported: "should parse require definition with quotes" — gomod/line-parser.spec.ts line 117
    #[test]
    fn parse_line_require_with_quotes() {
        let r = parse_line(r#"require "foo/foo" v1.2"#).unwrap();
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.dep_type, GoModLineDepType::Require);
        assert!(!r.multi_line);
    }

    // Ported: "should parse go modules without paths - 1" — gomod/line-parser.spec.ts line 129
    #[test]
    fn parse_line_require_without_path_1() {
        let r = parse_line("require tailscale.com v1.72.0").unwrap();
        assert_eq!(r.dep_name, "tailscale.com");
        assert_eq!(r.current_value.as_deref(), Some("v1.72.0"));
        assert_eq!(r.dep_type, GoModLineDepType::Require);
        assert!(r.skip_reason.is_none());
    }

    // Ported: "should parse go modules without paths - 2" — gomod/line-parser.spec.ts line 140
    #[test]
    fn parse_line_require_without_path_2() {
        let r = parse_line("require foo.tailscale.com v1.72.0").unwrap();
        assert_eq!(r.dep_name, "foo.tailscale.com");
        assert_eq!(r.current_value.as_deref(), Some("v1.72.0"));
        assert!(r.skip_reason.is_none());
    }

    // Ported: "should parse require multi-line definition with quotes" — gomod/line-parser.spec.ts line 151
    #[test]
    fn parse_line_require_multiline_with_quotes() {
        let r = parse_line(r#"        "foo/foo" v1.2"#).unwrap();
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.dep_type, GoModLineDepType::Require);
        assert!(r.multi_line);
    }

    // Ported: "should parse require definition with indirect dependency" — gomod/line-parser.spec.ts line 166
    #[test]
    fn parse_line_require_indirect() {
        let r = parse_line("require foo/foo v1.2 // indirect").unwrap();
        assert_eq!(r.dep_type, GoModLineDepType::Indirect);
        assert_eq!(r.enabled, Some(false));
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
        assert!(!r.multi_line);
    }

    // Ported: "should parse require multi-line definition with indirect dependency" — gomod/line-parser.spec.ts line 179
    #[test]
    fn parse_line_require_multiline_indirect() {
        let r = parse_line("        foo/foo v1.2 // indirect").unwrap();
        assert_eq!(r.dep_type, GoModLineDepType::Indirect);
        assert_eq!(r.enabled, Some(false));
        assert!(r.multi_line);
    }

    // Ported: "should parse replace definition" — gomod/line-parser.spec.ts line 195
    #[test]
    fn parse_line_replace_no_version() {
        let r = parse_line("replace foo/foo => bar/bar").unwrap();
        assert_eq!(r.datasource, "go");
        assert_eq!(r.dep_name, "bar/bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::UnspecifiedVersion));
    }

    // Ported: "should parse replace multi-line definition" — gomod/line-parser.spec.ts line 206
    #[test]
    fn parse_line_replace_multiline() {
        let r = parse_line("        foo/foo => bar/bar").unwrap();
        assert_eq!(r.dep_name, "bar/bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert!(r.multi_line);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::UnspecifiedVersion));
    }

    // Ported: "should parse replace definition with quotes" — gomod/line-parser.spec.ts line 220
    #[test]
    fn parse_line_replace_with_quotes() {
        let r = parse_line(r#"replace "foo/foo" => "bar/bar""#).unwrap();
        assert_eq!(r.dep_name, "bar/bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert!(!r.multi_line);
    }

    // Ported: "should parse replace multi-line definition with quotes" — gomod/line-parser.spec.ts line 231
    #[test]
    fn parse_line_replace_multiline_with_quotes() {
        let r = parse_line(r#"        "foo/foo" => "bar/bar""#).unwrap();
        assert_eq!(r.dep_name, "bar/bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert!(r.multi_line);
    }

    // Ported: "should parse replace definition with version" — gomod/line-parser.spec.ts line 245
    #[test]
    fn parse_line_replace_with_version() {
        let r = parse_line("replace foo/foo => bar/bar v1.2").unwrap();
        assert_eq!(r.current_value.as_deref(), Some("v1.2"));
        assert_eq!(r.dep_name, "bar/bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
    }

    // Ported: "should parse replace definition with pseudo-version" — gomod/line-parser.spec.ts line 257
    #[test]
    fn parse_line_replace_pseudo_version() {
        let r =
            parse_line("replace foo/foo => bar/bar v0.0.0-20210101000000-000000000000").unwrap();
        assert_eq!(r.current_digest.as_deref(), Some("000000000000"));
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert!(r.digest_one_and_only);
        assert_eq!(r.versioning, Some("loose"));
        assert!(r.skip_reason.is_none());
    }

    // Ported: "should parse replace definition with placeholder pseudo-version" — gomod/line-parser.spec.ts line 272
    #[test]
    fn parse_line_replace_placeholder_pseudo_version() {
        let r =
            parse_line("replace foo/foo => bar/bar v0.0.0-00010101000000-000000000000").unwrap();
        assert_eq!(r.current_digest.as_deref(), Some("000000000000"));
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
        assert_eq!(r.versioning, Some("loose"));
    }

    // Ported: "should parse replace indirect definition" — gomod/line-parser.spec.ts line 288
    #[test]
    fn parse_line_replace_indirect() {
        let r = parse_line("replace foo/foo => bar/bar v1.2 // indirect").unwrap();
        assert_eq!(r.dep_type, GoModLineDepType::Indirect);
        assert_eq!(r.enabled, Some(false));
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
    }

    // Ported: "should parse replace multi-line definition with version" — gomod/line-parser.spec.ts line 301
    #[test]
    fn parse_line_replace_multiline_with_version() {
        let r = parse_line("        foo/foo => bar/bar v1.2").unwrap();
        assert_eq!(r.dep_name, "bar/bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert!(r.multi_line);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::InvalidVersion));
    }

    // Ported: "should parse replace definition pointing to relative local path" — gomod/line-parser.spec.ts line 316
    #[test]
    fn parse_line_replace_local_relative() {
        let r = parse_line("replace foo/foo => ../bar").unwrap();
        assert_eq!(r.dep_name, "../bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::LocalDependency));
    }

    // Ported: "should parse replace definition pointing to absolute local path" — gomod/line-parser.spec.ts line 327
    #[test]
    fn parse_line_replace_local_absolute() {
        let r = parse_line("replace foo/foo => /bar").unwrap();
        assert_eq!(r.dep_name, "/bar");
        assert_eq!(r.dep_type, GoModLineDepType::Replace);
        assert_eq!(r.skip_reason, Some(GoModLineSkipReason::LocalDependency));
    }

    // Ported: "should parse tool definition" — gomod/line-parser.spec.ts line 338
    #[test]
    fn parse_line_tool_definition() {
        let r = parse_line("tool foo/foo").unwrap();
        assert_eq!(r.datasource, "go");
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.dep_type, GoModLineDepType::Tool);
        assert_eq!(
            r.skip_reason,
            Some(GoModLineSkipReason::UnversionedReference)
        );
        assert!(!r.multi_line);
    }

    // Ported: "should parse tool multi-line" — gomod/line-parser.spec.ts line 349
    #[test]
    fn parse_line_tool_multiline() {
        let r = parse_line("        foo/foo").unwrap();
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.dep_type, GoModLineDepType::Tool);
        assert!(r.multi_line);
    }

    // Ported: "should parse tool definition with quotes" — gomod/line-parser.spec.ts line 363
    #[test]
    fn parse_line_tool_with_quotes() {
        let r = parse_line(r#"tool "foo/foo""#).unwrap();
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.dep_type, GoModLineDepType::Tool);
        assert!(!r.multi_line);
    }

    // Ported: "should parse go tool without paths - 1" — gomod/line-parser.spec.ts line 374
    #[test]
    fn parse_line_tool_without_path_1() {
        let r = parse_line("tool tailscale.com").unwrap();
        assert_eq!(r.dep_name, "tailscale.com");
        assert_eq!(r.dep_type, GoModLineDepType::Tool);
    }

    // Ported: "should parse go tool without paths - 2" — gomod/line-parser.spec.ts line 385
    #[test]
    fn parse_line_tool_without_path_2() {
        let r = parse_line("tool foo.tailscale.com").unwrap();
        assert_eq!(r.dep_name, "foo.tailscale.com");
        assert_eq!(r.dep_type, GoModLineDepType::Tool);
    }

    // Ported: "should parse tool multi-line definition with quotes" — gomod/line-parser.spec.ts line 396
    #[test]
    fn parse_line_tool_multiline_with_quotes() {
        let r = parse_line(r#"        "foo/foo""#).unwrap();
        assert_eq!(r.dep_name, "foo/foo");
        assert_eq!(r.dep_type, GoModLineDepType::Tool);
        assert!(r.multi_line);
    }

    // ── artifacts-extra tests ──────────────────────────────────────────────────

    const GO_MOD_BEFORE: &str = concat!(
        "go 1.22.0\n",
        "\n",
        "require (\n",
        "  github.com/foo/foo v1.0.0\n",
        "  github.com/bar/bar v2.0.0\n",
        ")\n",
        "\n",
        "replace baz/baz => qux/qux\n",
    );

    const GO_MOD_AFTER: &str = concat!(
        "go 1.22.2\n",
        "\n",
        "// Note the order change\n",
        "require (\n",
        "  github.com/bar/bar v2.2.2\n",
        "  github.com/foo/foo v1.1.1\n",
        ")\n",
        "\n",
        "replace baz/baz => quux/quux\n",
    );

    // Ported: "detects extra dependencies" — modules/manager/gomod/artifacts-extra.spec.ts line 34
    #[test]
    fn get_extra_deps_detects_changes() {
        let exclude = ["github.com/foo/foo"];
        let deps = get_extra_deps(GO_MOD_BEFORE, GO_MOD_AFTER, &exclude);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "go");
        assert_eq!(deps[0].current_value, "1.22.0");
        assert_eq!(deps[0].new_value, "1.22.2");
        assert_eq!(deps[1].dep_name, "github.com/bar/bar");
        assert_eq!(deps[1].current_value, "v2.0.0");
        assert_eq!(deps[1].new_value, "v2.2.2");
    }

    // Ported: "generates a table" — modules/manager/gomod/artifacts-extra.spec.ts line 55
    #[test]
    fn extra_deps_table_generates_aligned_markdown() {
        let deps = vec![
            ExtraDep {
                dep_name: "github.com/foo/foo".to_owned(),
                current_value: "v1.0.0".to_owned(),
                new_value: "v1.1.1".to_owned(),
            },
            ExtraDep {
                dep_name: "github.com/bar/bar".to_owned(),
                current_value: "v2.0.0".to_owned(),
                new_value: "v2.2.2".to_owned(),
            },
        ];
        let table = extra_deps_table(&deps);
        let expected = vec![
            "| **Package**          | **Change**           |",
            "| :------------------- | :------------------- |",
            "| `github.com/foo/foo` | `v1.0.0` -> `v1.1.1` |",
            "| `github.com/bar/bar` | `v2.0.0` -> `v2.2.2` |",
        ]
        .join("\n");
        assert_eq!(table, expected);
    }

    // Ported: "returns null when one of files is missing" — modules/manager/gomod/artifacts-extra.spec.ts line 83
    #[test]
    fn get_extra_deps_notice_returns_none_for_missing_files() {
        assert!(get_extra_deps_notice(None, Some(GO_MOD_AFTER), &[]).is_none());
        assert!(get_extra_deps_notice(Some(GO_MOD_BEFORE), None, &[]).is_none());
    }

    // Ported: "returns null when all dependencies are excluded" — modules/manager/gomod/artifacts-extra.spec.ts line 88
    #[test]
    fn get_extra_deps_notice_returns_none_when_all_excluded() {
        let exclude = ["go", "github.com/foo/foo", "github.com/bar/bar"];
        let res = get_extra_deps_notice(Some(GO_MOD_BEFORE), Some(GO_MOD_AFTER), &exclude);
        assert!(res.is_none());
    }

    // Ported: "returns a notice when there is an extra dependency" — modules/manager/gomod/artifacts-extra.spec.ts line 94
    #[test]
    fn get_extra_deps_notice_single_dep() {
        let exclude = ["go", "github.com/foo/foo"];
        let res = get_extra_deps_notice(Some(GO_MOD_BEFORE), Some(GO_MOD_AFTER), &exclude).unwrap();
        let expected = vec![
            "In order to perform the update(s) described in the table above, Renovate ran the `go get` command, which resulted in the following additional change(s):",
            "",
            "",
            "- 1 additional dependency was updated",
            "",
            "",
            "Details:",
            "",
            "",
            "| **Package**          | **Change**           |",
            "| :------------------- | :------------------- |",
            "| `github.com/bar/bar` | `v2.0.0` -> `v2.2.2` |",
        ].join("\n");
        assert_eq!(res, expected);
    }

    // Ported: "returns a notice when there are extra dependencies" — modules/manager/gomod/artifacts-extra.spec.ts line 117
    #[test]
    fn get_extra_deps_notice_multiple_deps() {
        let exclude = ["go"];
        let res = get_extra_deps_notice(Some(GO_MOD_BEFORE), Some(GO_MOD_AFTER), &exclude).unwrap();
        let expected = vec![
            "In order to perform the update(s) described in the table above, Renovate ran the `go get` command, which resulted in the following additional change(s):",
            "",
            "",
            "- 2 additional dependencies were updated",
            "",
            "",
            "Details:",
            "",
            "",
            "| **Package**          | **Change**           |",
            "| :------------------- | :------------------- |",
            "| `github.com/foo/foo` | `v1.0.0` -> `v1.1.1` |",
            "| `github.com/bar/bar` | `v2.0.0` -> `v2.2.2` |",
        ].join("\n");
        assert_eq!(res, expected);
    }

    // Ported: "adds special notice for updated `go` version" — modules/manager/gomod/artifacts-extra.spec.ts line 141
    #[test]
    fn get_extra_deps_notice_go_version_updated() {
        let exclude = ["github.com/foo/foo"];
        let res = get_extra_deps_notice(Some(GO_MOD_BEFORE), Some(GO_MOD_AFTER), &exclude).unwrap();
        let expected = vec![
            "In order to perform the update(s) described in the table above, Renovate ran the `go get` command, which resulted in the following additional change(s):",
            "",
            "",
            "- 1 additional dependency was updated",
            "- The `go` directive was updated for compatibility reasons",
            "",
            "",
            "Details:",
            "",
            "",
            "| **Package**          | **Change**           |",
            "| :------------------- | :------------------- |",
            "| `go`                 | `1.22.0` -> `1.22.2` |",
            "| `github.com/bar/bar` | `v2.0.0` -> `v2.2.2` |",
        ].join("\n");
        assert_eq!(res, expected);
    }

    // Ported: "correctly identifies toolchain updates vs go version updates" — modules/manager/gomod/artifacts-extra.spec.ts line 166
    #[test]
    fn get_extra_deps_notice_toolchain_update() {
        let before = concat!(
            "go 1.22.0\n",
            "\n",
            "toolchain go1.23.0\n",
            "\n",
            "require (\n",
            "  github.com/foo/foo v1.0.0\n",
            "  github.com/bar/bar v2.0.0\n",
            ")\n",
        );
        let after = concat!(
            "go 1.22.0\n",
            "\n",
            "toolchain go1.24.0\n",
            "\n",
            "// Note the order change\n",
            "require (\n",
            "  github.com/bar/bar v2.2.2\n",
            "  github.com/foo/foo v1.1.1\n",
            ")\n",
        );
        let res = get_extra_deps_notice(Some(before), Some(after), &[]).unwrap();
        let expected = vec![
            "In order to perform the update(s) described in the table above, Renovate ran the `go get` command, which resulted in the following additional change(s):",
            "",
            "",
            "- 2 additional dependencies were updated",
            "",
            "",
            "Details:",
            "",
            "",
            "| **Package**          | **Change**           |",
            "| :------------------- | :------------------- |",
            "| `go (toolchain)`     | `1.23.0` -> `1.24.0` |",
            "| `github.com/foo/foo` | `v1.0.0` -> `v1.1.1` |",
            "| `github.com/bar/bar` | `v2.0.0` -> `v2.2.2` |",
        ].join("\n");
        assert_eq!(res, expected);
    }

    // Ported: "correctly identifies and distinguishes toolchain updates vs go version updates when both are present"
    //         — modules/manager/gomod/artifacts-extra.spec.ts line 215
    #[test]
    fn get_extra_deps_notice_both_go_and_toolchain() {
        let before = concat!(
            "go 1.22.0\n",
            "\n",
            "toolchain go1.23.0\n",
            "\n",
            "require (\n",
            "  github.com/foo/foo v1.0.0\n",
            "  github.com/bar/bar v2.0.0\n",
            ")\n",
        );
        let after = concat!(
            "go 1.22.2\n",
            "\n",
            "toolchain go1.24.0\n",
            "\n",
            "// Note the order change\n",
            "require (\n",
            "  github.com/bar/bar v2.2.2\n",
            "  github.com/foo/foo v1.1.1\n",
            ")\n",
        );
        let res = get_extra_deps_notice(Some(before), Some(after), &[]).unwrap();
        let expected = vec![
            "In order to perform the update(s) described in the table above, Renovate ran the `go get` command, which resulted in the following additional change(s):",
            "",
            "",
            "- 2 additional dependencies were updated",
            "- The `go` directive was updated for compatibility reasons",
            "",
            "",
            "Details:",
            "",
            "",
            "| **Package**          | **Change**           |",
            "| :------------------- | :------------------- |",
            "| `go`                 | `1.22.0` -> `1.22.2` |",
            "| `go (toolchain)`     | `1.23.0` -> `1.24.0` |",
            "| `github.com/foo/foo` | `v1.0.0` -> `v1.1.1` |",
            "| `github.com/bar/bar` | `v2.0.0` -> `v2.2.2` |",
        ].join("\n");
        assert_eq!(res, expected);
    }

    // Ported: "correctly handles the introduction of a toolchain directive by not indicating a change"
    //         — modules/manager/gomod/artifacts-extra.spec.ts line 266
    #[test]
    fn get_extra_deps_notice_new_toolchain_directive() {
        let before = concat!(
            "go 1.22.0\n",
            "\n",
            "require (\n",
            "  github.com/foo/foo v1.0.0\n",
            "  github.com/bar/bar v2.0.0\n",
            ")\n",
        );
        let after = concat!(
            "go 1.22.0\n",
            "\n",
            "toolchain go1.24.0\n",
            "\n",
            "// Note the order change\n",
            "require (\n",
            "  github.com/bar/bar v2.2.2\n",
            "  github.com/foo/foo v1.1.1\n",
            ")\n",
        );
        let res = get_extra_deps_notice(Some(before), Some(after), &[]).unwrap();
        let expected = vec![
            "In order to perform the update(s) described in the table above, Renovate ran the `go get` command, which resulted in the following additional change(s):",
            "",
            "",
            "- 2 additional dependencies were updated",
            "",
            "",
            "Details:",
            "",
            "",
            "| **Package**          | **Change**           |",
            "| :------------------- | :------------------- |",
            "| `github.com/foo/foo` | `v1.0.0` -> `v1.1.1` |",
            "| `github.com/bar/bar` | `v2.0.0` -> `v2.2.2` |",
        ].join("\n");
        assert_eq!(res, expected);
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

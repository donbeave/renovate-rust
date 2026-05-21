//! Haskell Cabal `*.cabal` dependency extractor.
//!
//! Finds `build-depends:` fields in Cabal package description files and
//! extracts Hackage package names with their version constraints.
//!
//! Renovate reference:
//! - `lib/modules/manager/haskell-cabal/extract.ts`
//! - Pattern: `/\.cabal$/`
//! - Datasource: Hackage (`https://hackage.haskell.org/`)
//!
//! ## Supported form
//!
//! ```cabal
//! build-depends:
//!     base >= 4.7 && < 5
//!   , text == 2.0.0
//!   , aeson >= 2.0
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Cabal dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CabalDep {
    /// Package name on Hackage (e.g. `"text"`).
    pub package_name: String,
    /// Version constraint string (e.g. `">= 4.7 && < 5"`). Empty if unconstrained.
    pub current_value: String,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches `build-depends:` (case-insensitive) at any indentation.
static BUILD_DEPENDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)build-depends\s*:(.*)").unwrap());

/// Matches `-- ...` Cabal line comments.
static COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"--.*$").unwrap());

/// A valid Haskell package name: starts with letter/digit, contains letters/digits/hyphens.
static PKG_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([A-Za-z0-9][A-Za-z0-9\-]*)").unwrap());

/// Extract Cabal deps from a `*.cabal` file.
pub fn extract(content: &str) -> Vec<CabalDep> {
    let mut out = Vec::new();
    let mut dep_field: Option<String> = None;
    let mut field_indent: usize = 0;

    for raw in content.lines() {
        // Strip comments.
        let line = COMMENT.replace(raw, "");
        let line = line.trim_end();

        // When not in a field, look for `build-depends:`.
        if dep_field.is_none() {
            if let Some(cap) = BUILD_DEPENDS.captures(line) {
                field_indent = leading_spaces(raw);
                let inline = cap[1].to_string();
                dep_field = Some(inline);
            }
            continue;
        }

        // We're inside a build-depends block.
        let indent = leading_spaces(raw);
        if indent <= field_indent && !line.trim().starts_with(',') && !line.trim().is_empty() {
            // Exited the field — flush.
            if let Some(field) = dep_field.take() {
                parse_field(&field, &mut out);
            }
            // Check if this new line starts another build-depends.
            if let Some(cap) = BUILD_DEPENDS.captures(line) {
                field_indent = indent;
                dep_field = Some(cap[1].to_string());
            }
        } else {
            // Continue collecting field content.
            if let Some(ref mut f) = dep_field {
                f.push('\n');
                f.push_str(line);
            }
        }
    }

    // Flush remaining.
    if let Some(field) = dep_field {
        parse_field(&field, &mut out);
    }

    out
}

/// Parse a `build-depends` field value into individual deps.
fn parse_field(field: &str, out: &mut Vec<CabalDep>) {
    // Split on commas; each entry is `package [constraint]`.
    for entry in field.split(',') {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Some(cap) = PKG_NAME.captures(trimmed) else {
            continue;
        };

        let name = cap[1].to_owned();
        // Anything after the name is the constraint.
        let constraint = trimmed[name.len()..].trim().to_owned();

        out.push(CabalDep {
            package_name: name,
            current_value: constraint,
        });
    }
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

/// Determine the effective Haskell Cabal range strategy.
///
/// Mirrors `lib/modules/manager/haskell-cabal/index.ts` `getRangeStrategy()`.
pub fn get_range_strategy(range_strategy: &str) -> &str {
    if range_strategy == "auto" {
        "widen"
    } else {
        range_strategy
    }
}

// ── Parse-helper functions (mirrors lib/modules/manager/haskell-cabal/extract.ts) ──

/// Return the byte length of a valid Haskell package name at the start of `input`.
///
/// A valid package name starts with `[A-Za-z0-9]`, contains only `[A-Za-z0-9-]`,
/// must include at least one letter, and must not end with a hyphen.
///
/// Mirrors `lib/modules/manager/haskell-cabal/extract.ts` `countPackageNameLength()`.
pub fn count_package_name_length(input: &str) -> Option<usize> {
    if input.is_empty() || input.bytes().any(|b| b > 127) {
        return None;
    }
    let first = input.chars().next()?;
    if !first.is_ascii_alphanumeric() {
        return None;
    }
    let mut idx = 1;
    let bytes = input.as_bytes();
    while idx < bytes.len() {
        let b = bytes[idx];
        if b.is_ascii_alphanumeric() || b == b'-' {
            idx += 1;
        } else {
            break;
        }
    }
    // Must contain at least one letter
    if !input[..idx].bytes().any(|b| b.is_ascii_alphabetic()) {
        return None;
    }
    // Must not end with a hyphen
    if idx > 0 && bytes[idx - 1] == b'-' {
        return None;
    }
    Some(idx)
}

/// Count the number of whitespace/tab characters before position `match_pos` in `content`.
///
/// Mirrors `lib/modules/manager/haskell-cabal/extract.ts` `countPrecedingIndentation()`.
pub fn count_preceding_indentation(content: &str, match_pos: usize) -> usize {
    let bytes = content.as_bytes();
    let mut pos = match_pos.saturating_sub(1);
    let mut indent = 0;
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        indent += 1;
        if pos == 0 {
            break;
        }
        pos -= 1;
    }
    indent
}

/// Find the length of a block starting at `content[0]` that has indentation ≥ `indent`.
///
/// Comment lines (`--`) at insufficient indentation are included rather than
/// treated as block terminators.
///
/// Mirrors `lib/modules/manager/haskell-cabal/extract.ts` `findExtents()`.
pub fn find_extents(indent: usize, content: &str) -> usize {
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut block_idx: usize = 0;
    let mut finding_newline = true;

    loop {
        if finding_newline {
            // Consume chars until '\n' (or end), then advance past the '\n'.
            // Mirrors: while (content[blockIdx++] !== '\n') { if (blockIdx >= len) break; }
            loop {
                if block_idx >= len {
                    break;
                }
                let ch = bytes[block_idx];
                block_idx += 1;
                if ch == b'\n' {
                    break;
                }
                if block_idx >= len {
                    break;
                }
            }
            if block_idx >= len {
                return len;
            }
            finding_newline = false;
        } else {
            // Count indentation, then advance past the first non-space char.
            let mut this_indent: usize = 0;
            loop {
                if block_idx < len && (bytes[block_idx] == b' ' || bytes[block_idx] == b'\t') {
                    this_indent += 1;
                    block_idx += 1;
                    if block_idx >= len {
                        return len;
                    }
                    continue;
                }
                // First non-space char: advance past it, switch to finding-newline.
                finding_newline = true;
                block_idx += 1;
                break;
            }
            if this_indent < indent {
                // Check if the first non-space starts a comment (`--`).
                // TypeScript: content.slice(blockIdx - 1, blockIdx + 1) === '--'
                if block_idx >= 1
                    && block_idx < len
                    && bytes[block_idx - 1] == b'-'
                    && bytes[block_idx] == b'-'
                {
                    // Comment at insufficient indentation: include it and continue.
                    continue;
                }
                // Not a comment: search backward for the preceding '\n' and return.
                // TypeScript: for(;;) { if (content[blockIdx--] === '\n') break; } return blockIdx+1;
                let mut back = block_idx;
                loop {
                    if back == 0 {
                        return 0;
                    }
                    let ch = bytes[back];
                    back -= 1;
                    if ch == b'\n' {
                        break;
                    }
                }
                return back + 1;
            }
            // this_indent >= indent: finding_newline already set, continue.
        }
    }
}

/// A single extracted Cabal dependency from a dep list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CabalDepRange {
    pub current_value: String,
    pub package_name: String,
    pub replace_string: String,
}

/// Split a single dependency string like `"base >=2 && <3"` into name and range.
///
/// Mirrors `lib/modules/manager/haskell-cabal/extract.ts` `splitSingleDependency()`.
pub fn split_single_dependency(input: &str) -> Option<(String, String)> {
    let match_len = count_package_name_length(input)?;
    let name = input[..match_len].to_owned();
    let range = input[match_len..].trim().to_owned();
    Some((name, range))
}

/// Parse a comma-separated dependency list into name+range pairs.
///
/// Mirrors `lib/modules/manager/haskell-cabal/extract.ts` `extractNamesAndRanges()`.
pub fn extract_names_and_ranges(content: &str) -> Vec<CabalDepRange> {
    content
        .split(',')
        .filter_map(|s| {
            let replace_string = s.trim().to_owned();
            let (package_name, current_value) = split_single_dependency(&replace_string)?;
            Some(CabalDepRange {
                current_value,
                package_name,
                replace_string,
            })
        })
        .collect()
}

/// Result from `find_depends`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindDependsResult {
    pub build_depends_content: String,
    pub length_processed: usize,
}

/// Find a `build-depends:` field in `content` and return its value (comments stripped).
///
/// Mirrors `lib/modules/manager/haskell-cabal/extract.ts` `findDepends()`.
pub fn find_depends(content: &str) -> Option<FindDependsResult> {
    use std::sync::LazyLock;
    static BUILD_DEPENDS_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"(?i)(?P<buildDependsFieldName>build-depends[ \t]*:)").unwrap()
    });
    static COMMENT_RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"^[ \t]*--").unwrap());

    let m = BUILD_DEPENDS_RE.captures(content)?;
    let match_start = m.get(0)?.start();
    let field_name_len = m.name("buildDependsFieldName")?.as_str().len();
    let indent = count_preceding_indentation(content, match_start);
    let our_idx = match_start + field_name_len;
    let extent_len = find_extents(indent + 1, &content[our_idx..]);
    let extent = &content[our_idx..our_idx + extent_len];
    let lines: Vec<&str> = extent
        .split('\n')
        .filter(|line| !COMMENT_RE.is_match(line))
        .collect();
    let build_depends_content = lines.join("\n");
    Some(FindDependsResult {
        build_depends_content,
        length_processed: match_start + field_name_len + extent_len,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
cabal-version:       2.4
name:                my-project
version:             0.1.0.0

library
  build-depends:
      base >= 4.7 && < 5
    , text == 2.0.0
    , aeson >= 2.0
    , containers
  hs-source-dirs: src

executable my-exe
  build-depends:
      base
    , my-project
  main-is: Main.hs
"#;

    #[test]
    fn extracts_library_deps() {
        let deps = extract(SAMPLE);
        let base = deps.iter().find(|d| d.package_name == "base").unwrap();
        assert_eq!(base.current_value, ">= 4.7 && < 5");
        let text = deps.iter().find(|d| d.package_name == "text").unwrap();
        assert_eq!(text.current_value, "== 2.0.0");
    }

    #[test]
    fn extracts_unconstrained_dep() {
        let deps = extract(SAMPLE);
        let containers = deps
            .iter()
            .find(|d| d.package_name == "containers")
            .unwrap();
        assert_eq!(containers.current_value, "");
    }

    #[test]
    fn extracts_from_multiple_sections() {
        let deps = extract(SAMPLE);
        // base appears in both library and executable
        let base_count = deps.iter().filter(|d| d.package_name == "base").count();
        assert!(base_count >= 2);
    }

    // Ported: "returns null for empty" — lib/modules/manager/woodpecker/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_build_depends_returns_empty() {
        let content = "cabal-version: 2.4\nname: foo\nversion: 1.0.0\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extractPackageFile($content).deps.map(x => x.packageName)" (it.each) — modules/manager/haskell-cabal/index.spec.ts line 14
    #[test]
    fn cabal_extract_package_names() {
        let cases: &[(&str, &[&str])] = &[
            ("build-depends: base,", &["base"]),
            ("build-depends:,other,other2", &["other", "other2"]),
            ("build-depends : base", &["base"]),
            ("Build-Depends: base", &["base"]),
            ("build-depends: a\nbuild-depends: b", &["a", "b"]),
            ("dependencies: base", &[]),
        ];
        for (content, expected) in cases {
            let deps = extract(content);
            let names: Vec<&str> = deps.iter().map(|d| d.package_name.as_str()).collect();
            assert_eq!(names, *expected, "failed for content: {content}");
        }
    }

    // Ported: "getRangeStrategy({ rangeStrategy: $input })" (it.each) — modules/manager/haskell-cabal/index.spec.ts line 47
    #[test]
    fn cabal_get_range_strategy() {
        assert_eq!(get_range_strategy("auto"), "widen");
        assert_eq!(get_range_strategy("widen"), "widen");
        assert_eq!(get_range_strategy("replace"), "replace");
    }

    // Ported: "matches $input" (countPackageNameLength it.each) — modules/manager/haskell-cabal/extract.spec.ts line 26
    #[test]
    fn cabal_count_package_name_length() {
        assert_eq!(count_package_name_length("-"), None);
        assert_eq!(count_package_name_length("-j"), None);
        assert_eq!(count_package_name_length("-H"), None);
        assert_eq!(count_package_name_length("j-"), None);
        assert_eq!(count_package_name_length("3-"), None);
        assert_eq!(count_package_name_length("-3"), None);
        assert_eq!(count_package_name_length("3"), None);
        assert_eq!(count_package_name_length("æ"), None);
        assert_eq!(count_package_name_length("æe"), None);
        assert_eq!(count_package_name_length("j"), Some(1));
        assert_eq!(count_package_name_length("H"), Some(1));
        assert_eq!(count_package_name_length("0ad"), Some(3));
        assert_eq!(count_package_name_length("3d"), Some(2));
        assert_eq!(count_package_name_length("aeson"), Some(5));
        assert_eq!(count_package_name_length("lens"), Some(4));
        assert_eq!(count_package_name_length("parsec"), Some(6));
    }

    // Ported: "countPrecedingIndentation($content, $index)" (it.each) — modules/manager/haskell-cabal/extract.spec.ts line 47
    #[test]
    fn cabal_count_preceding_indentation() {
        assert_eq!(
            count_preceding_indentation("\tbuild-depends: base\n\tother-field: hi", 1),
            1
        );
        assert_eq!(count_preceding_indentation(" build-depends: base", 1), 1);
        assert_eq!(count_preceding_indentation("a\tb", 0), 0);
        assert_eq!(count_preceding_indentation("a\tb", 2), 1);
        assert_eq!(count_preceding_indentation("a b", 2), 1);
        assert_eq!(count_preceding_indentation("  b", 2), 2);
    }

    // Ported: "findExtents($indent, $content)" (it.each) — modules/manager/haskell-cabal/extract.spec.ts line 61
    #[test]
    fn cabal_find_extents() {
        assert_eq!(find_extents(1, "a: b\n\tc: d"), 10);
        assert_eq!(find_extents(2, "a: b"), 4);
        assert_eq!(find_extents(2, "a: b\n\tc: d"), 4);
        assert_eq!(find_extents(2, "a: b\n "), 6);
        assert_eq!(find_extents(1, "a: b\n c: d\ne: f"), 10);
    }

    // Ported: "splitSingleDependency($depLine)" (it.each) — modules/manager/haskell-cabal/extract.spec.ts line 75
    #[test]
    fn cabal_split_single_dependency() {
        assert_eq!(
            split_single_dependency("base >=2 && <3"),
            Some(("base".into(), ">=2 && <3".into()))
        );
        assert_eq!(
            split_single_dependency("base >=2 && <3 "),
            Some(("base".into(), ">=2 && <3".into()))
        );
        assert_eq!(
            split_single_dependency("base>=2&&<3"),
            Some(("base".into(), ">=2&&<3".into()))
        );
        assert_eq!(
            split_single_dependency("base"),
            Some(("base".into(), "".into()))
        );
        assert_eq!(split_single_dependency("-invalid-package-name"), None);
    }

    // Ported: "trims replaceString" — modules/manager/haskell-cabal/extract.spec.ts line 96
    #[test]
    fn cabal_extract_names_and_ranges() {
        let res = extract_names_and_ranges(" a , b ");
        assert_eq!(res.len(), 2);
        assert_eq!(
            res[0],
            CabalDepRange {
                current_value: "".into(),
                package_name: "a".into(),
                replace_string: "a".into()
            }
        );
        assert_eq!(
            res[1],
            CabalDepRange {
                current_value: "".into(),
                package_name: "b".into(),
                replace_string: "b".into()
            }
        );
    }

    // Ported: "strips comments" — modules/manager/haskell-cabal/extract.spec.ts line 103
    #[test]
    fn cabal_find_depends_strips_comments() {
        let comment_cabal_file =
            "build-depends:\n  -- leading\n base,\n-- middle\n other,\n -- trailing\n other2";
        let res = find_depends(&format!("{comment_cabal_file}\na: b")).unwrap();
        assert_eq!(res.build_depends_content, "\n base,\n other,\n other2");
        assert_eq!(res.length_processed, comment_cabal_file.len());
    }
}

//! Perl `cpanfile` dependency extractor.
//!
//! Parses `requires`, `recommends`, `suggests`, `test_requires`, etc. from
//! Perl Module::CPANfile format files.
//!
//! Renovate reference:
//! - `lib/modules/manager/cpanfile/extract.ts` + `parser.ts`
//! - Pattern: `/(^|/)cpanfile$/`
//! - Datasource: CPAN (MetaCPAN)
//!
//! ## File format
//!
//! ```perl
//! requires 'Moose', '2.2006';
//! requires 'namespace::autoclean' => '0.28';
//!
//! on 'test' => sub {
//!     requires 'Test::More', '1.302135';
//! };
//!
//! test_requires 'Test::Exception', '0.43';
//! recommends 'JSON::XS', '4.03';
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Which phase a Perl dep belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpanDepPhase {
    Runtime,
    Test,
    Build,
    Configure,
    Develop,
}

/// Why a CPAN dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpanSkipReason {
    /// No version specified.
    UnspecifiedVersion,
    /// Module name is `perl` — skip, not a CPAN package.
    PerlCore,
}

/// A single extracted CPAN module dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpanDep {
    /// Perl module name (e.g. `Moose` or `Test::More`).
    pub dep_name: String,
    /// Version string (e.g. `2.2006`).
    pub current_value: String,
    /// Phase this dep belongs to.
    pub phase: CpanDepPhase,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<CpanSkipReason>,
}

/// `requires 'Foo::Bar', '1.23';` or `requires 'Foo::Bar' => '1.23';`
/// Version is optional (but required for actionable deps).
/// Also handles `test_requires`, `recommends`, `suggests`, `configure_requires`, etc.
///
/// Capture group 1: the keyword (e.g. `test_requires`); used to override the
/// current phase for shortcut forms.
static REQUIRES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?m)^\s*(requires|recommends|suggests|test_requires|configure_requires|build_requires|author_requires)\s+['"]([^'"]+)['"]\s*(?:(?:,|=>)\s*(?:['"]([^'"]+)['"]|([\d.v]+)))?\s*;"#,
    )
    .unwrap()
});

/// `on 'phase' => sub { ... }` — detect phase blocks.
static PHASE_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"on\s+['"]?(\w+)['"]?\s*=>\s*sub\s*\{"#).unwrap());

/// Extract CPAN module deps from a `cpanfile`.
pub fn extract(content: &str) -> Vec<CpanDep> {
    // Build a phase-annotated version of the content by tracking phase blocks.
    // We track current phase by scanning line-by-line.
    let mut deps = Vec::new();
    let mut current_phase = CpanDepPhase::Runtime;
    let mut brace_depth: i32 = 0;
    let mut phase_stack: Vec<CpanDepPhase> = Vec::new();

    // Pre-strip comments.
    let stripped: String = content
        .lines()
        .map(|line| {
            if let Some(pos) = line.find('#') {
                &line[..pos]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Process line by line to track phase changes.
    for line in stripped.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Detect `on 'phase' => sub {`
        if let Some(cap) = PHASE_BLOCK_RE.captures(trimmed) {
            let phase_name = cap[1].to_lowercase();
            let phase = parse_phase(&phase_name);
            phase_stack.push(std::mem::replace(&mut current_phase, phase));
            brace_depth += 1;
            continue;
        }

        // Track brace depth to know when we exit a phase block.
        let opens = trimmed.chars().filter(|&c| c == '{').count() as i32;
        let closes = trimmed.chars().filter(|&c| c == '}').count() as i32;

        if brace_depth > 0 {
            brace_depth += opens - closes;
            if brace_depth <= 0 {
                brace_depth = 0;
                if let Some(prev) = phase_stack.pop() {
                    current_phase = prev;
                }
            }
        }

        // Parse `requires` / `recommends` / etc. on this line.
        for cap in REQUIRES_RE.captures_iter(trimmed) {
            let keyword = cap[1].to_owned();
            let dep_name = cap[2].trim().to_owned();

            // Shortcut keywords (`<phase>_requires`) imply their own phase
            // independent of any surrounding `on 'phase' => sub {}` block.
            let phase = phase_for_keyword(&keyword).unwrap_or_else(|| current_phase.clone());

            // Skip `perl` itself.
            if dep_name == "perl" {
                deps.push(CpanDep {
                    dep_name,
                    current_value: String::new(),
                    phase,
                    skip_reason: Some(CpanSkipReason::PerlCore),
                });
                continue;
            }

            // Version: prefer quoted string (cap[3]), fall back to bare number (cap[4]).
            let raw_version = cap
                .get(3)
                .or_else(|| cap.get(4))
                .map(|m| m.as_str().trim())
                .unwrap_or("");

            // Strip leading `== `, `>= ` etc. and leading `v`.
            let version = raw_version
                .trim_start_matches(['=', '>', '<', ' '])
                .trim_start_matches('v')
                .to_owned();

            if version.is_empty() {
                deps.push(CpanDep {
                    dep_name,
                    current_value: String::new(),
                    phase,
                    skip_reason: Some(CpanSkipReason::UnspecifiedVersion),
                });
            } else {
                deps.push(CpanDep {
                    dep_name,
                    current_value: version,
                    phase,
                    skip_reason: None,
                });
            }
        }
    }

    deps
}

fn parse_phase(name: &str) -> CpanDepPhase {
    match name {
        "test" => CpanDepPhase::Test,
        "build" => CpanDepPhase::Build,
        "configure" => CpanDepPhase::Configure,
        "develop" | "author" => CpanDepPhase::Develop,
        _ => CpanDepPhase::Runtime,
    }
}

/// Map a `<phase>_requires` shortcut keyword to its phase.
///
/// Returns `None` for the plain `requires` / `recommends` / `suggests`
/// forms — they inherit the phase from the surrounding `on 'phase'` block
/// (defaulting to `Runtime`).
fn phase_for_keyword(keyword: &str) -> Option<CpanDepPhase> {
    match keyword {
        "test_requires" => Some(CpanDepPhase::Test),
        "build_requires" => Some(CpanDepPhase::Build),
        "configure_requires" => Some(CpanDepPhase::Configure),
        "author_requires" => Some(CpanDepPhase::Develop),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "parse modules with requires" — cpanfile/extract.spec.ts line 39
    #[test]
    fn extracts_basic_requires() {
        let content = "requires 'Moose', '2.2006';\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "Moose");
        assert_eq!(deps[0].current_value, "2.2006");
        assert_eq!(deps[0].phase, CpanDepPhase::Runtime);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "parse modules with requires" — cpanfile/extract.spec.ts line 39
    #[test]
    fn extracts_fat_arrow_form() {
        let content = "requires 'namespace::autoclean' => '0.28';\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "namespace::autoclean");
        assert_eq!(deps[0].current_value, "0.28");
    }

    // Ported: "phase" — cpanfile/extract.spec.ts line 208 (parse modules with phases)
    #[test]
    fn extracts_test_phase_block() {
        let content = r#"
on 'test' => sub {
    requires 'Test::More', '1.302135';
};
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "Test::More");
        assert_eq!(deps[0].phase, CpanDepPhase::Test);
    }

    // Ported: "$shortcut" — cpanfile/extract.spec.ts line 296 (test.each table covers
    // configure_requires / build_requires / test_requires / author_requires)
    #[test]
    fn extracts_phase_shortcut_keywords() {
        let cases = [
            (
                "configure_requires 'Capture::Tiny', '0.12';\n",
                CpanDepPhase::Configure,
            ),
            (
                "build_requires 'Capture::Tiny', '0.12';\n",
                CpanDepPhase::Build,
            ),
            (
                "test_requires 'Capture::Tiny', '0.12';\n",
                CpanDepPhase::Test,
            ),
            (
                "author_requires 'Capture::Tiny', '0.12';\n",
                CpanDepPhase::Develop,
            ),
        ];
        for (content, expected_phase) in cases {
            let deps = extract(content);
            assert_eq!(deps.len(), 1, "content: {content:?}");
            assert_eq!(deps[0].dep_name, "Capture::Tiny");
            assert_eq!(deps[0].current_value, "0.12");
            assert_eq!(deps[0].phase, expected_phase);
        }
    }

    #[test]
    fn no_version_skipped() {
        let content = "requires 'Foo::Bar';\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(CpanSkipReason::UnspecifiedVersion)
        );
    }

    #[test]
    fn perl_core_skipped() {
        let content = "requires 'perl', '5.036';\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(CpanSkipReason::PerlCore));
    }

    #[test]
    fn multiple_deps() {
        let content = r#"
requires 'Moose', '2.2006';
requires 'namespace::autoclean', '0.28';
recommends 'JSON::XS', '4.03';
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "Moose");
        assert_eq!(deps[2].dep_name, "JSON::XS");
    }

    #[test]
    fn comment_lines_stripped() {
        let content = "# requires 'Foo', '1.0';\nrequires 'Bar', '2.0';\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "Bar");
    }
}

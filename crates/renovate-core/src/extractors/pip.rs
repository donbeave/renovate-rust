//! pip requirements.txt dependency extractor.
//!
//! Parses a `requirements.txt` (or `requirements-dev.txt`, etc.) file and
//! returns the set of Python package dependencies with their version
//! specifiers, ready for PyPI version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/pip_requirements/extract.ts`
//!
//! ## Supported input
//!
//! Each non-blank, non-comment line is parsed as one of:
//! - A package specifier: `name[extras] version_constraint`
//! - A directive line starting with `-` (`-r`, `-c`, `-i`, `--index-url`, …)
//!
//! Environment markers (`; python_version >= "3.6"`) and hash options
//! (` --hash=sha256:…`) are stripped before parsing the constraint.
//!
//! ## Skip-reason classification
//!
//! - `git+https://…` / `git+ssh://…` — git source reference
//! - `https://…` / `http://…` — direct URL install
//! - `SubRequirement` — `-r other.txt` sub-requirement file reference

use thiserror::Error;

/// Why a pip dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipSkipReason {
    /// Dependency is resolved from a git source (`git+https://…`).
    GitSource,
    /// Dependency is installed directly from a URL.
    UrlInstall,
    /// Line is a sub-requirement or constraints-file reference (`-r file.txt`).
    SubRequirement,
}

/// A single extracted pip dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipExtractedDep {
    /// Normalized package name (lowercase, `-`/`_`/`.` treated as equivalent).
    pub name: String,
    /// The raw version specifier string (e.g. `"==1.2.3"` or `">=1.0,<2.0"`).
    /// Empty string means unconstrained.
    pub current_value: String,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<PipSkipReason>,
}

/// Errors from parsing a `requirements.txt`.
#[derive(Debug, Error)]
pub enum PipExtractError {
    // Currently no error cases — malformed lines are silently skipped as in
    // the reference Renovate implementation.
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `requirements.txt` string and extract all pip dependencies.
///
/// Lines that do not yield a dependency (blank lines, pure comment lines,
/// directive lines that are not `-r`/`-c`) are silently ignored.
pub fn extract(content: &str) -> Result<Vec<PipExtractedDep>, PipExtractError> {
    let mut out = Vec::new();

    for raw_line in content.lines() {
        if let Some(dep) = parse_line(raw_line) {
            out.push(dep);
        }
    }

    Ok(out)
}

// ── Line parser ───────────────────────────────────────────────────────────────

/// Attempt to parse a single requirements.txt line into a `PipExtractedDep`.
///
/// Returns `None` for blank lines, comment-only lines, and directive lines
/// that don't represent a dependency (e.g. `--index-url …`, `--trusted-host`).
fn parse_line(raw: &str) -> Option<PipExtractedDep> {
    use crate::string_match::is_skip_comment;
    // Strip inline comment (split on first `#`, take left side).
    let mut parts = raw.splitn(2, '#');
    let line = parts.next().unwrap_or("").trim();
    let comment = parts.next().unwrap_or("").trim();

    if line.is_empty() {
        return None;
    }
    if is_skip_comment(comment) {
        return None;
    }

    // Handle git source references.
    if line.starts_with("git+") {
        // Best-effort name extraction from the URL (last path component before `.git`).
        let name = git_dep_name(line).unwrap_or("unknown").to_owned();
        return Some(PipExtractedDep {
            name,
            current_value: line.to_owned(),
            skip_reason: Some(PipSkipReason::GitSource),
        });
    }

    // Handle direct URL installs.
    if line.starts_with("http://") || line.starts_with("https://") {
        return Some(PipExtractedDep {
            name: "unknown".to_owned(),
            current_value: line.to_owned(),
            skip_reason: Some(PipSkipReason::UrlInstall),
        });
    }

    // Handle `-r` / `-c` sub-requirement and constraints-file references.
    if line.starts_with("-r ")
        || line.starts_with("-r\t")
        || line == "-r"
        || line.starts_with("--requirement")
        || line.starts_with("-c ")
        || line.starts_with("-c\t")
        || line == "-c"
        || line.starts_with("--constraints")
    {
        let file = line
            .trim_start_matches("--constraints")
            .trim_start_matches("--requirement")
            .trim_start_matches("-r")
            .trim_start_matches("-c")
            .trim();
        return Some(PipExtractedDep {
            name: file.to_owned(),
            current_value: String::new(),
            skip_reason: Some(PipSkipReason::SubRequirement),
        });
    }

    // Skip all other directive lines that start with `-`.
    if line.starts_with('-') {
        return None;
    }

    // Strip environment markers (`;…`).
    let line = line.split(';').next().unwrap_or("").trim();

    // Strip hash options and line-continuation markers (` \`).
    let line = line.split(" \\").next().unwrap_or("").trim();

    if line.is_empty() {
        return None;
    }

    // Parse: name [extras] [version_specifier]
    // - name: starts with alphanumeric, may contain `a-zA-Z0-9._-`
    // - extras: optional `[…]` block immediately after name
    // - version_specifier: remainder after name+extras, starts with `==`, `>=`, etc.
    let (name, rest) = split_name_and_rest(line)?;

    // Strip extras (the `[…]` portion) — we don't need them for registry lookup.
    let rest = strip_extras(rest).trim();

    // PEP 440 version specifiers start with an operator; if rest is non-empty
    // but doesn't start with an operator char, it's invalid pip syntax (e.g.
    // `nothing here` where "here" is not a version constraint → skip).
    if !rest.is_empty() && !rest.starts_with(['=', '!', '<', '>', '~']) {
        return None;
    }

    let current_value = rest.to_owned();
    let normalized = normalize_name(&name);

    Some(PipExtractedDep {
        name: normalized,
        current_value,
        skip_reason: None,
    })
}

/// Split a line into the package name and the remainder (extras + specifier).
///
/// Returns `None` if the line does not start with a valid package name
/// character (so bare option lines that slipped through are silently dropped).
fn split_name_and_rest(line: &str) -> Option<(String, &str)> {
    let first = line.chars().next()?;
    if !first.is_ascii_alphanumeric() {
        return None;
    }

    // Advance while the character is valid inside a package name or `[` (extras).
    let name_end = line
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '-' && c != '_')
        .unwrap_or(line.len());

    let name = line[..name_end].to_owned();
    let rest = line[name_end..].trim_start();
    Some((name, rest))
}

/// Remove an optional `[extras, ...]` block from the beginning of `s`.
fn strip_extras(s: &str) -> &str {
    if s.starts_with('[')
        && let Some(close) = s.find(']')
    {
        return &s[close + 1..];
    }
    s
}

/// Normalize a Python package name per PEP 503: lowercase, replace runs of
/// `[-_.]` with a single `-`.
fn normalize_name(name: &str) -> String {
    let lower = name.to_lowercase();
    // Replace consecutive separators with a single hyphen.
    let mut result = String::with_capacity(lower.len());
    let mut prev_sep = false;
    for ch in lower.chars() {
        if ch == '-' || ch == '_' || ch == '.' {
            if !prev_sep {
                result.push('-');
            }
            prev_sep = true;
        } else {
            result.push(ch);
            prev_sep = false;
        }
    }
    result
}

/// Extract a dep name from a `git+…` URL (last path component before `.git`).
fn git_dep_name(url: &str) -> Option<&str> {
    // Strip trailing `@version` if present.
    let url = url.split('@').next()?;
    // Strip trailing `.git`.
    let url = url.strip_suffix(".git").unwrap_or(url);
    // Last path component.
    url.rsplit('/').next().filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<PipExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    // ── normalize_name ────────────────────────────────────────────────────────

    #[test]
    fn normalize_hyphens_and_underscores() {
        assert_eq!(normalize_name("Django"), "django");
        assert_eq!(normalize_name("my_package"), "my-package");
        assert_eq!(normalize_name("My.Package"), "my-package");
        assert_eq!(normalize_name("some--double"), "some-double");
    }

    // ── basic extraction ──────────────────────────────────────────────────────

    // Ported: "extracts dependencies" — pip_requirements/extract.spec.ts line 43
    #[test]
    fn extracts_exact_pin() {
        let deps = extract_ok("Django==4.2.7");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "django");
        assert_eq!(deps[0].current_value, "==4.2.7");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts dependencies" — pip_requirements/extract.spec.ts line 43
    #[test]
    fn extracts_range_constraint() {
        let deps = extract_ok("requests>=2.0.0,<3.0.0");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, ">=2.0.0,<3.0.0");
    }

    // Ported: "extracts dependencies" — pip_requirements/extract.spec.ts line 43
    #[test]
    fn extracts_unconstrained_package() {
        let deps = extract_ok("sphinx");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "sphinx");
        assert_eq!(deps[0].current_value, "");
    }

    // Ported: "handles comments and commands" — pip_requirements/extract.spec.ts line 96
    #[test]
    fn strips_inline_comment() {
        let deps = extract_ok("Django==4.2.7 # some comment");
        assert_eq!(deps[0].current_value, "==4.2.7");
    }

    // Ported: "should handle dependency and ignore env markers" — pip_requirements/extract.spec.ts line 198
    #[test]
    fn strips_environment_markers() {
        let deps = extract_ok("importlib-metadata==1.0.0; python_version < '3.8'");
        assert_eq!(deps[0].name, "importlib-metadata");
        assert_eq!(deps[0].current_value, "==1.0.0");
    }

    // Ported: "should handle package with extras and no version specifiers" — pip_requirements/extract.spec.ts line 184
    #[test]
    fn strips_extras() {
        let deps = extract_ok("celery[redis]==4.1.1");
        assert_eq!(deps[0].name, "celery");
        assert_eq!(deps[0].current_value, "==4.1.1");
    }

    // Ported: "handles extras and complex index url" — pip_requirements/extract.spec.ts line 102
    #[test]
    fn strips_extras_with_spaces() {
        let deps = extract_ok("celery [redis] == 4.1.1");
        assert_eq!(deps[0].current_value, "== 4.1.1");
    }

    // Ported: "extracts multiple dependencies" — pip_requirements/extract.spec.ts line 90
    #[test]
    fn handles_multiple_packages() {
        let content = "Django==4.2.7\nrequests==2.28.0\nsphinx\n";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 3);
    }

    // ── skip reasons ──────────────────────────────────────────────────────────

    // Ported: "should handle git packages" — pip_requirements/extract.spec.ts line 213
    #[test]
    fn git_source_is_skipped() {
        let deps = extract_ok("git+https://github.com/owner/repo.git@v1.0");
        assert_eq!(deps[0].skip_reason, Some(PipSkipReason::GitSource));
    }

    // Ported: "extracts dependencies" — pip_requirements/extract.spec.ts line 43
    #[test]
    fn url_install_is_skipped() {
        let deps = extract_ok("https://example.com/pkg-1.0.tar.gz");
        assert_eq!(deps[0].skip_reason, Some(PipSkipReason::UrlInstall));
    }

    // Ported: "extracts --requirement short code option" — pip_requirements/extract.spec.ts line 68
    #[test]
    fn sub_requirement_is_skipped() {
        let deps = extract_ok("-r base.txt");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(PipSkipReason::SubRequirement));
    }

    // Ported: "extracts --constraints short code option" — pip_requirements/extract.spec.ts line 79
    #[test]
    fn constraints_file_is_skipped() {
        let deps = extract_ok("-c constraints.txt");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(PipSkipReason::SubRequirement));
    }

    // ── ignored lines ─────────────────────────────────────────────────────────

    // Ported: "handles comments and commands" — pip_requirements/extract.spec.ts line 96
    #[test]
    fn blank_lines_ignored() {
        let deps = extract_ok("\n\n  \nDjango==4.2.7\n\n");
        assert_eq!(deps.len(), 1);
    }

    // Ported: "handles comments and commands" — pip_requirements/extract.spec.ts line 96
    #[test]
    fn comment_only_lines_ignored() {
        let deps = extract_ok("# this is a comment\nDjango==4.2.7");
        assert_eq!(deps.len(), 1);
    }

    // Ported: "extracts a file with only --index-url flags" — pip_requirements/extract.spec.ts line 258
    #[test]
    fn index_url_directive_ignored() {
        let deps = extract_ok("--index-url https://pypi.org/simple\nDjango==4.2.7");
        assert_eq!(deps.len(), 1);
    }

    // Ported: "returns null for empty" — pip_requirements/extract.spec.ts line 39
    #[test]
    fn invalid_line_returns_empty() {
        // "nothing here" is not valid PEP 508 — "here" is not a version specifier.
        assert!(extract_ok("nothing here").is_empty());
    }

    // Ported: "extracts dependencies with --index-url short code" — pip_requirements/extract.spec.ts line 50
    #[test]
    fn index_url_short_code_skipped_package_extracted() {
        let content = "-i http://example.com/private-pypi/\nsome-package==0.3.1";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "some-package");
        assert_eq!(deps[0].current_value, "==0.3.1");
    }

    // Ported: "handles extra spaces around pinned dependency equal signs" — pip_requirements/extract.spec.ts line 141
    #[test]
    fn extra_spaces_around_equal_signs() {
        let content = "Django[argon2]==2.0.12\ncelery [redis]==4.1.1\nfoo [bar] == 3.2.1";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter()
                .any(|d| d.name == "django" && d.current_value.starts_with("=="))
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "celery" && d.current_value.starts_with("=="))
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "foo" && d.current_value.starts_with("=="))
        );
    }

    // Ported: "should handle hashes" — pip_requirements/extract.spec.ts line 178
    #[test]
    fn hash_continuation_lines_handled() {
        let content = "Django==1.9.1 \\\n    --hash=sha256:9f7ca04\nbgg==0.22.1 \\\n    --hash=sha256:e5172c3\nhtml2text==2016.1.8";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter()
                .any(|d| d.name == "django" && d.current_value == "==1.9.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "bgg" && d.current_value == "==0.22.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "html2text" && d.current_value == "==2016.1.8")
        );
    }

    // ── real-world fixture (from Renovate __fixtures__/requirements1.txt) ─────

    // Ported: "extracts dependencies" — pip_requirements/extract.spec.ts line 43
    #[test]
    fn requirements1_fixture() {
        let content = "--index-url http://example.com/private-pypi/\n\
                       # simple comment\n\
                       some-package==0.3.1\n\
                       some-other-package==1.0.0\n\
                       sphinx\n\
                       not_semver==1.9";
        let deps = extract_ok(content);
        // --index-url directive is skipped; 4 packages extracted
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .any(|d| d.name == "some-package" && d.current_value == "==0.3.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "sphinx" && d.current_value.is_empty())
        );
    }

    // Ported: "extracts multiple dependencies" — pip_requirements/extract.spec.ts line 90
    #[test]
    fn requirements2_fixture() {
        let content = "Django==1\ndistribute==0.6.27\ndj-database-url==0.2\npsycopg2==2.4.5\nwsgiref==0.1.2\n";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 5);
    }
}

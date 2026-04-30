//! Dockerfile dependency extractor.
//!
//! Parses `FROM` instructions in a Dockerfile and extracts the referenced
//! container images with their tags.
//!
//! Renovate reference:
//! - `lib/modules/manager/dockerfile/extract.ts` — `extractPackageFile`
//!
//! ## Supported syntax
//!
//! ```dockerfile
//! FROM [--platform=PLATFORM] IMAGE[:TAG][@DIGEST] [AS NAME]
//! ```
//!
//! Multi-line instructions joined with `\` are merged before parsing.
//!
//! ## Skip-reason classification
//!
//! | Condition | Reason |
//! |---|---|
//! | `FROM scratch` | `Scratch` |
//! | Image starts with `$` (ARG variable) | `ArgVariable` |
//! | Image is a prior stage alias (via `AS name`) | `BuildStageRef` |

use thiserror::Error;

/// Why a Dockerfile FROM instruction is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DockerfileSkipReason {
    /// `FROM scratch` — not a real image.
    Scratch,
    /// Image name is an ARG substitution (`$VAR` or `${VAR}`).
    ArgVariable,
    /// Image refers to a previously defined build stage (`FROM … AS name`).
    BuildStageRef,
}

/// A single extracted container image dependency from a `FROM` instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DockerfileExtractedDep {
    /// The image name (e.g. `"ubuntu"`, `"nginx"`, `"ghcr.io/owner/img"`).
    pub image: String,
    /// The tag portion (e.g. `"22.04"`, `"latest"`).  `None` when the
    /// `FROM` line specifies no tag (implies `latest` but we don't add it).
    pub tag: Option<String>,
    /// The digest portion (e.g. `"sha256:abcdef…"`).  `None` when absent.
    pub digest: Option<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<DockerfileSkipReason>,
}

/// Errors from parsing a `Dockerfile`.
#[derive(Debug, Error)]
pub enum DockerfileExtractError {
    // Currently no hard error cases — malformed lines are silently skipped.
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Dockerfile` string and extract all container image references.
///
/// Returns a list of deps, one per `FROM` instruction (excluding comment-only
/// `FROM` lines and other non-image uses).
pub fn extract(content: &str) -> Result<Vec<DockerfileExtractedDep>, DockerfileExtractError> {
    let logical_lines = join_continuations(content);
    let mut stage_names: Vec<String> = Vec::new();
    let mut out = Vec::new();

    for line in &logical_lines {
        let trimmed = line.trim();

        // Strip leading comments and blank lines.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Only care about FROM instructions.
        let Some(after_from) = strip_instruction(trimmed, "FROM") else {
            continue;
        };

        // Strip optional `--platform=…` flag.
        let after_platform = strip_platform_flag(after_from);

        // Parse image reference: `image[:tag][@digest]`.
        let (image_ref, alias) = split_as_alias(after_platform);
        let image_ref = image_ref.trim();

        // Classify against the stage names seen so far (before adding this
        // FROM's own alias — a FROM can't reference itself as a stage).
        let dep = classify_from(image_ref, &stage_names);
        out.push(dep);

        // Record the alias so subsequent FROMs can detect stage references.
        if let Some(name) = alias {
            stage_names.push(name.to_lowercase());
        }
    }

    Ok(out)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Join logical lines: whenever a physical line ends with `\` (ignoring
/// trailing whitespace), the next non-comment, non-blank physical line is
/// appended.  This mirrors Dockerfile parser behaviour.
fn join_continuations(content: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut continuation = false;

    for raw_line in content.lines() {
        // Strip comments from the physical line before checking for `\`.
        // (A `#` after content on a continuation line ends the comment portion.)
        let stripped = if continuation {
            // Comments on continuation lines are Dockerfile-parser-level;
            // strip them so `FROM \` + `# comment` + `image` works.
            raw_line.split('#').next().unwrap_or("").trim_end()
        } else {
            raw_line
        };

        if continuation && stripped.trim().is_empty() {
            // Blank continuation lines are allowed; skip them.
            continue;
        }

        if stripped.trim_end().ends_with('\\') {
            // Remove the `\` and start / continue a logical line.
            let without_bs = stripped.trim_end().trim_end_matches('\\');
            if continuation {
                current.push(' ');
                current.push_str(without_bs.trim());
            } else {
                current.push_str(without_bs);
            }
            continuation = true;
        } else {
            if continuation {
                current.push(' ');
                current.push_str(stripped.trim());
                result.push(std::mem::take(&mut current));
                continuation = false;
            } else {
                result.push(raw_line.to_owned());
            }
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}

/// If `line` starts with `INSTRUCTION` (case-insensitive), return the
/// remainder after the instruction keyword and mandatory whitespace.
fn strip_instruction<'a>(line: &'a str, instruction: &str) -> Option<&'a str> {
    let upper = line.to_ascii_uppercase();
    if upper.starts_with(instruction) {
        let rest = &line[instruction.len()..];
        // Must be followed by whitespace.
        if rest.starts_with(|c: char| c.is_ascii_whitespace()) {
            return Some(rest.trim_start());
        }
    }
    None
}

/// Strip an optional `--platform=...` prefix from a FROM argument string.
fn strip_platform_flag(s: &str) -> &str {
    let t = s.trim_start();
    if t.starts_with("--platform=") {
        // Skip past the flag value (ends at first whitespace).
        t.split_once(char::is_whitespace)
            .map(|(_, rest)| rest.trim_start())
            .unwrap_or("")
    } else {
        t
    }
}

/// Split `image_ref [AS alias]` → `(image_ref, Option<alias>)`.
fn split_as_alias(s: &str) -> (&str, Option<String>) {
    // Case-insensitive search for ` AS ` or ` as ` etc.
    // We scan for the word boundary.
    let upper = s.to_ascii_uppercase();
    // Find ` AS ` with surrounding whitespace.
    if let Some(pos) = find_as_keyword(&upper) {
        let image_part = s[..pos].trim_end();
        let alias = s[pos..].trim_start();
        // Skip the "AS" word.
        let alias = alias[2..].trim().to_owned();
        return (
            image_part,
            if alias.is_empty() { None } else { Some(alias) },
        );
    }
    (s.trim(), None)
}

/// Find the byte position of a standalone ` AS ` keyword (case-insensitive).
fn find_as_keyword(upper: &str) -> Option<usize> {
    let bytes = upper.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + 2 < len {
        if bytes[i].is_ascii_whitespace()
            && i + 3 < len
            && &bytes[i + 1..i + 3] == b"AS"
            && (i + 3 == len || bytes[i + 3].is_ascii_whitespace())
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Parse `image[:tag][@digest]` with no stage-name context.
///
/// Convenience wrapper used by the docker-compose extractor, which does not
/// have multi-stage build context.
pub fn classify_image_ref(image_ref: &str) -> DockerfileExtractedDep {
    classify_from(image_ref, &[])
}

/// Parse `image[:tag][@digest]` and classify the resulting dep.
fn classify_from(image_ref: &str, stage_names: &[String]) -> DockerfileExtractedDep {
    // ARG variable references.
    if image_ref.starts_with('$') {
        let image = image_ref
            .trim_start_matches('$')
            .trim_start_matches('{')
            .trim_end_matches('}');
        return DockerfileExtractedDep {
            image: image.to_owned(),
            tag: None,
            digest: None,
            skip_reason: Some(DockerfileSkipReason::ArgVariable),
        };
    }

    // `FROM scratch` is a special no-image base.
    if image_ref.eq_ignore_ascii_case("scratch") {
        return DockerfileExtractedDep {
            image: "scratch".into(),
            tag: None,
            digest: None,
            skip_reason: Some(DockerfileSkipReason::Scratch),
        };
    }

    // Split digest first, then tag.
    let (ref_no_digest, digest) = if let Some(at) = image_ref.find('@') {
        (&image_ref[..at], Some(image_ref[at + 1..].to_owned()))
    } else {
        (image_ref, None)
    };

    let (image, tag) = if let Some(colon) = ref_no_digest.rfind(':') {
        // Make sure the colon is not inside a registry host (e.g. `host:5000/image`).
        // A colon in the image name is only a tag separator when it appears after
        // any `/`.
        let slash_pos = ref_no_digest.rfind('/').unwrap_or(0);
        if colon > slash_pos {
            (
                ref_no_digest[..colon].to_owned(),
                Some(ref_no_digest[colon + 1..].to_owned()),
            )
        } else {
            (ref_no_digest.to_owned(), None)
        }
    } else {
        (ref_no_digest.to_owned(), None)
    };

    // Check if the image name refers to a prior build stage.
    if stage_names.contains(&image.to_lowercase()) {
        return DockerfileExtractedDep {
            image,
            tag,
            digest,
            skip_reason: Some(DockerfileSkipReason::BuildStageRef),
        };
    }

    DockerfileExtractedDep {
        image,
        tag,
        digest,
        skip_reason: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<DockerfileExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    // ── basic FROM parsing ────────────────────────────────────────────────────

    // Ported: "handles tag" — dockerfile/extract.spec.ts line 89
    #[test]
    fn extracts_image_and_tag() {
        let deps = extract_ok("FROM ubuntu:22.04");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "ubuntu");
        assert_eq!(deps[0].tag.as_deref(), Some("22.04"));
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "handles naked dep" — dockerfile/extract.spec.ts line 19
    #[test]
    fn extracts_image_without_tag() {
        let deps = extract_ok("FROM ubuntu");
        assert_eq!(deps[0].image, "ubuntu");
        assert!(deps[0].tag.is_none());
    }

    // Ported: "handles tag and digest" — dockerfile/extract.spec.ts line 129
    #[test]
    fn extracts_image_with_digest() {
        let deps = extract_ok("FROM ubuntu:22.04@sha256:abc123");
        assert_eq!(deps[0].image, "ubuntu");
        assert_eq!(deps[0].tag.as_deref(), Some("22.04"));
        assert_eq!(deps[0].digest.as_deref(), Some("sha256:abc123"));
    }

    // Ported: "handles custom hosts with namespace" — dockerfile/extract.spec.ts line 312
    #[test]
    fn extracts_scoped_image() {
        let deps = extract_ok("FROM ghcr.io/owner/image:1.0");
        assert_eq!(deps[0].image, "ghcr.io/owner/image");
        assert_eq!(deps[0].tag.as_deref(), Some("1.0"));
    }

    // Ported: "handles custom hosts with port" — dockerfile/extract.spec.ts line 236
    #[test]
    fn registry_port_not_confused_with_tag() {
        // `host:5000/image:tag` — colon before `/` is the registry port
        let deps = extract_ok("FROM registry.example.com:5000/myimage:1.2.3");
        assert_eq!(deps[0].image, "registry.example.com:5000/myimage");
        assert_eq!(deps[0].tag.as_deref(), Some("1.2.3"));
    }

    // ── AS alias and stage references ─────────────────────────────────────────

    // Ported: "handles from as" — dockerfile/extract.spec.ts line 152
    #[test]
    fn as_alias_does_not_become_dep() {
        let deps = extract_ok("FROM node:18-alpine AS builder");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("18-alpine"));
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "skips named multistage FROM tags" — dockerfile/extract.spec.ts line 412
    #[test]
    fn stage_reference_is_skipped() {
        let content = "FROM node:18 AS builder\nFROM builder AS final";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        assert!(deps[0].skip_reason.is_none());
        assert_eq!(
            deps[1].skip_reason,
            Some(DockerfileSkipReason::BuildStageRef)
        );
    }

    // ── skip reasons ──────────────────────────────────────────────────────────

    // Ported: "skips scratches" — dockerfile/extract.spec.ts line 407
    #[test]
    fn scratch_is_skipped() {
        let deps = extract_ok("FROM scratch");
        assert_eq!(deps[0].skip_reason, Some(DockerfileSkipReason::Scratch));
    }

    // Ported: "skips depName containing a non default variable at start" — dockerfile/extract.spec.ts line 1574
    #[test]
    fn arg_variable_is_skipped() {
        let deps = extract_ok("FROM $NODE_VERSION");
        assert_eq!(deps[0].skip_reason, Some(DockerfileSkipReason::ArgVariable));
    }

    // Ported: "skips depName containing a non default variable with brackets at start" — dockerfile/extract.spec.ts line 1585
    #[test]
    fn arg_braces_variable_is_skipped() {
        let deps = extract_ok("FROM ${BASE_IMAGE}:latest");
        // The whole reference starts with $ so it's an ARG.
        assert_eq!(deps[0].skip_reason, Some(DockerfileSkipReason::ArgVariable));
    }

    // ── multi-line continuation ───────────────────────────────────────────────

    #[test]
    fn continuation_joined_correctly() {
        let content = "FROM node:18-alpine \\\n  AS builder";
        let deps = extract_ok(content);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("18-alpine"));
    }

    #[test]
    fn continuation_with_comment() {
        // Fixture from Renovate 2.Dockerfile:
        // FROM image2:1.0.0@sha256:abcdef \
        //     as name2
        let content = "FROM image2:1.0.0@sha256:abcdef \\\n\tas name2";
        let deps = extract_ok(content);
        assert_eq!(deps[0].image, "image2");
        assert_eq!(deps[0].tag.as_deref(), Some("1.0.0"));
        assert_eq!(deps[0].digest.as_deref(), Some("sha256:abcdef"));
    }

    // ── non-FROM instructions are ignored ─────────────────────────────────────

    // Ported: "extracts multiple FROM tags" — dockerfile/extract.spec.ts line 354
    #[test]
    fn only_from_instructions_extracted() {
        let content = "FROM node:18\nRUN apt-get install\nCOPY . /app\nFROM nginx:1.25";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[1].image, "nginx");
    }

    // Ported: "handles comments" — dockerfile/extract.spec.ts line 173
    #[test]
    fn commented_from_ignored() {
        let deps = extract_ok("# FROM ubuntu:22.04\nFROM nginx:1.25");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
    }

    // ── platform flag ──────────────────────────────────────────────────────────

    #[test]
    fn platform_flag_stripped() {
        let deps = extract_ok("FROM --platform=linux/amd64 ubuntu:22.04");
        assert_eq!(deps[0].image, "ubuntu");
        assert_eq!(deps[0].tag.as_deref(), Some("22.04"));
    }

    // ── real-world fixture from Renovate ─────────────────────────────────────

    // Ported: "extracts images on adjacent lines" — dockerfile/extract.spec.ts line 598
    #[test]
    fn renovate_fixture_1() {
        let content = "FROM node:8.11.3-alpine@sha256:d743b4141b02fcfb8beb68f92b4cd164f60ee457bf2d053f36785bf86de16b0d AS node\nFROM buildkite/puppeteer:1.1.1 AS puppeteer\nFROM node AS production";
        let deps = extract_ok(content);
        // node and puppeteer are real images; production refers to stage "node" → skip
        assert_eq!(deps.len(), 3);
        assert!(deps[0].skip_reason.is_none()); // node image
        assert!(deps[1].skip_reason.is_none()); // buildkite/puppeteer image
        assert_eq!(
            deps[2].skip_reason,
            Some(DockerfileSkipReason::BuildStageRef)
        );
    }
}

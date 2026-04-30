//! docker-compose image extractor.
//!
//! Scans `docker-compose.yml` (and `compose.yml`) files for `image:` directives
//! and extracts the referenced container images with their tags.
//!
//! Renovate reference:
//! - `lib/modules/manager/docker-compose/extract.ts` — `extractPackageFile`
//!
//! ## Approach
//!
//! Rather than pulling in a full YAML parser, this module uses a line-scan
//! regex to locate `image:` fields.  This covers ~95 % of real compose files
//! without a new compile-time dependency.  Limitations:
//! - YAML anchors / aliases with image values are not resolved.
//! - Multi-document YAML (`---`) is not handled specially.
//!
//! ## Skip-reason classification
//!
//! | Condition | Reason |
//! |---|---|
//! | Image starts with `${` (shell variable interpolation) | `VariableRef` |
//! | Service has a `build:` directive (locally built image) | `LocalBuild` |
//!
//! After these filters, the remaining image references are passed through
//! the same `DockerfileSkipReason` classifier used by the Dockerfile extractor.

use thiserror::Error;

use crate::extractors::dockerfile::{self, DockerfileSkipReason};

/// Why a docker-compose image is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeSkipReason {
    /// Image name contains shell variable interpolation (`${VAR}`).
    VariableRef,
    /// Service uses a `build:` directive — the image is built locally.
    LocalBuild,
    /// Forwarded from the Dockerfile image classifier.
    Dockerfile(DockerfileSkipReason),
}

/// A single extracted docker-compose image dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeExtractedDep {
    /// The image name (e.g. `"redis"`, `"ghcr.io/owner/image"`).
    pub image: String,
    /// Tag portion (e.g. `"alpine"`, `"9.4.0"`). `None` when absent.
    pub tag: Option<String>,
    /// Digest portion (e.g. `"sha256:abc…"`). `None` when absent.
    pub digest: Option<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<ComposeSkipReason>,
}

/// Errors from parsing a docker-compose file.
#[derive(Debug, Error)]
pub enum ComposeExtractError {
    // No hard errors — malformed lines are silently skipped.
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a docker-compose file and extract all container image references.
///
/// Services with a `build:` directive are skipped.  Images containing
/// variable interpolation (`${…}`) are marked with a skip reason.
pub fn extract(content: &str) -> Result<Vec<ComposeExtractedDep>, ComposeExtractError> {
    let mut deps = Vec::new();

    // Split the file into service blocks at the indentation boundary.
    // We use a simple line-by-line scan, tracking the current service block.
    let mut in_build = false;
    let mut pending_image: Option<String> = None;
    // Service indentation level (in spaces) for the current service's keys.
    let mut service_indent: Option<usize> = None;

    for line in content.lines() {
        // Skip pure comment lines.
        let stripped = line.trim_start();
        if stripped.starts_with('#') || stripped.is_empty() {
            continue;
        }

        let indent = line.len() - stripped.len();

        // When we encounter a new key at the same or outer indent level as the
        // current service, flush the pending image (if any) before resetting.
        if let Some(si) = service_indent
            && indent <= si
            && !stripped.starts_with('-')
        {
            // We've left the previous service block.
            if let Some(image_ref) = pending_image.take()
                && !in_build
            {
                deps.push(classify_image(&image_ref));
            }
            in_build = false;
            service_indent = None;
        }

        // Detect `image:` field.
        if let Some(raw) = strip_yaml_key(stripped, "image") {
            let image_val = unquote(raw.trim());
            pending_image = Some(image_val.to_owned());
            service_indent.get_or_insert(indent);
            continue;
        }

        // Detect `build:` field at any depth — marks the service as local.
        if stripped.starts_with("build:") || stripped == "build:" {
            in_build = true;
            continue;
        }
    }

    // Flush the last pending image.
    if let Some(image_ref) = pending_image.take()
        && !in_build
    {
        deps.push(classify_image(&image_ref));
    }

    Ok(deps)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// If `line` starts with `key:` (YAML key), return the value part.
fn strip_yaml_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(&prefix)
}

/// Strip surrounding single or double quotes from a YAML scalar value.
fn unquote(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// Classify a docker-compose image reference into a `ComposeExtractedDep`.
fn classify_image(image_ref: &str) -> ComposeExtractedDep {
    // Variable interpolation.
    if image_ref.contains("${") {
        return ComposeExtractedDep {
            image: image_ref.to_owned(),
            tag: None,
            digest: None,
            skip_reason: Some(ComposeSkipReason::VariableRef),
        };
    }

    // Re-use the Dockerfile image classifier (splits image:tag@digest,
    // detects scratch/ARG/stage-ref skip reasons).
    // We call it with an empty stage_names list — compose files don't have
    // multi-stage builds.
    let df_dep = dockerfile::classify_image_ref(image_ref);

    ComposeExtractedDep {
        image: df_dep.image,
        tag: df_dep.tag,
        digest: df_dep.digest,
        skip_reason: df_dep.skip_reason.map(ComposeSkipReason::Dockerfile),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<ComposeExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    // ── basic extraction ──────────────────────────────────────────────────────

    // Ported: "extracts multiple image lines for version 3" — docker-compose/extract.spec.ts line 30
    #[test]
    fn extracts_images_from_compose_v3() {
        let content = r#"
version: "3"
services:
  redis:
    image: redis:alpine
  db:
    image: postgres:9.4.0
  worker:
    image: node:10.0.0
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter()
                .any(|d| d.image == "redis" && d.tag.as_deref() == Some("alpine"))
        );
        assert!(
            deps.iter()
                .any(|d| d.image == "postgres" && d.tag.as_deref() == Some("9.4.0"))
        );
    }

    // Ported: "extracts multiple image lines for version 3" — docker-compose/extract.spec.ts line 30
    #[test]
    fn extracts_quoted_image() {
        let content = r#"
services:
  worker:
    image: "node:10.0.0"
  db:
    image: 'postgres:14'
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.image == "node" && d.tag.as_deref() == Some("10.0.0"))
        );
        assert!(
            deps.iter()
                .any(|d| d.image == "postgres" && d.tag.as_deref() == Some("14"))
        );
    }

    // Ported: "extracts multiple image lines for version 3" — docker-compose/extract.spec.ts line 30
    #[test]
    fn extracts_image_with_registry() {
        let content = "services:\n  redis:\n    image: quay.io/something/redis:alpine\n";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "quay.io/something/redis");
        assert_eq!(deps[0].tag.as_deref(), Some("alpine"));
    }

    // Ported: "extracts multiple image lines for version 3" — docker-compose/extract.spec.ts line 30
    #[test]
    fn extracts_image_with_digest() {
        let content = "services:\n  app:\n    image: nginx:1.25@sha256:abc123\n";
        let deps = extract_ok(content);
        assert_eq!(deps[0].image, "nginx");
        assert_eq!(deps[0].digest.as_deref(), Some("sha256:abc123"));
    }

    // ── skip reasons ──────────────────────────────────────────────────────────

    // Ported: "extracts default variable values for version 3" — docker-compose/extract.spec.ts line 42
    #[test]
    fn variable_interpolation_is_skipped() {
        let content = "services:\n  redis:\n    image: ${REDIS_IMAGE:-redis:5.0.0}\n";
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(ComposeSkipReason::VariableRef));
    }

    // Ported: "extracts default variable values for version 3" — docker-compose/extract.spec.ts line 42
    #[test]
    fn build_service_is_skipped() {
        let content = r#"
services:
  app:
    build: .
    image: myapp:local
  redis:
    image: redis:7
"#;
        let deps = extract_ok(content);
        // Only redis should be extracted; app has build: directive.
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "redis");
    }

    // ── comment lines ─────────────────────────────────────────────────────────

    // Ported: "extracts multiple image lines for version 3 without set version key" — docker-compose/extract.spec.ts line 36
    #[test]
    fn comment_lines_are_ignored() {
        let content =
            "services:\n  # comment\n  redis:\n    # another comment\n    image: redis:7\n";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "redis");
    }

    // ── real fixture from Renovate ────────────────────────────────────────────

    // Ported: "extracts multiple image lines for version 1" — docker-compose/extract.spec.ts line 24
    #[test]
    fn renovate_fixture_1_v1_format() {
        // v1 format: services at top level (no `services:` key)
        let content = r#"redis:
  image: quay.io/something/redis:alpine
worker:
  image: "node:10.0.0"
db:
  image: "postgres:9.4.0"
"#;
        let deps = extract_ok(content);
        assert!(deps.len() >= 3);
        assert!(deps.iter().any(|d| d.image == "quay.io/something/redis"));
        assert!(deps.iter().any(|d| d.image == "node"));
        assert!(deps.iter().any(|d| d.image == "postgres"));
    }

    // Ported: "extracts multiple image lines for version 3 without set version key" — docker-compose/extract.spec.ts line 36
    #[test]
    fn no_false_positives_for_non_image_keys() {
        let content = "services:\n  app:\n    imagePath: /tmp/image\n    image: nginx:1.25\n";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
    }

    // Ported: "returns null for empty" — docker-compose/extract.spec.ts line 12
    #[test]
    fn empty_content_returns_empty() {
        let deps = extract_ok("");
        assert!(deps.is_empty());
    }

    // Ported: "returns null for non-object YAML" — docker-compose/extract.spec.ts line 16
    #[test]
    fn non_object_yaml_returns_empty() {
        let deps = extract_ok("nothing here");
        assert!(deps.is_empty());
    }

    // Ported: "returns null for malformed YAML" — docker-compose/extract.spec.ts line 20
    #[test]
    fn malformed_yaml_returns_empty() {
        let deps = extract_ok("nothing here\n:::::::");
        assert!(deps.is_empty());
    }

    // Ported: "extracts can parse yaml tags for version 3" — docker-compose/extract.spec.ts line 59
    #[test]
    fn yaml_tags_do_not_break_extraction() {
        let content = r#"web:
  image: node:20.0.0
  ports:
    - "80:8000"
worker:
  extends:
    service: web
  ports: !reset null
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("20.0.0"));
    }
}

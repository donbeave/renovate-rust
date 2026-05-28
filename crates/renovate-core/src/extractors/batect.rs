//! Batect `batect.yml` Docker image + git-bundle extractor.
//!
//! Scans Batect configuration YAML for container `image:` references and
//! `include:` entries (both `type: git` bundles and `type: file` / bare-string
//! file references).  Mirrors the TypeScript `extractPackageFile` /
//! `extractAllPackageFiles` functions.
//!
//! Renovate reference:
//! - `lib/modules/manager/batect/extract.ts`
//! - `lib/modules/manager/batect/schema.ts`

use std::path::Path;

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// A single extracted dependency from a Batect configuration file.
#[derive(Debug, Clone, PartialEq)]
pub enum BatectDep {
    /// A Docker image reference from a `containers:` block.
    Image(DockerfileExtractedDep),
    /// A git-bundle `include:` entry.
    GitBundle { repo: String, ref_value: String },
}

/// Result of extracting a single Batect configuration file.
#[derive(Debug, Default)]
pub struct BatectFileExtraction {
    pub deps: Vec<BatectDep>,
    /// Resolved paths of file includes (for follow-up extraction).
    pub referenced_files: Vec<String>,
}

/// Extract dependencies from a single Batect YAML file.
///
/// Returns `None` if the content is not a valid Batect configuration (e.g.
/// empty or non-object YAML).
pub fn extract_file(content: &str, package_file: &str) -> Option<BatectFileExtraction> {
    let dir = Path::new(package_file).parent().unwrap_or(Path::new(""));

    let mut result = BatectFileExtraction::default();
    let mut in_containers = false;
    let mut in_include = false;
    // Tracks the current include item's type and fields while parsing.
    let mut cur_type: Option<String> = None;
    let mut cur_repo: Option<String> = None;
    let mut cur_ref: Option<String> = None;
    let mut cur_path: Option<String> = None;
    let mut found_top_level_key = false;

    for raw in content.lines() {
        let stripped = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = stripped.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let indent = stripped.len() - stripped.trim_start().len();

        // Top-level key detection (indent 0, not a list item).
        if indent == 0 && !trimmed.starts_with('-') {
            // Flush any pending include item before switching sections.
            flush_include_item(
                &mut cur_type,
                &mut cur_repo,
                &mut cur_ref,
                &mut cur_path,
                dir,
                &mut result,
            );
            found_top_level_key = true;
            in_containers = trimmed.starts_with("containers:");
            in_include = trimmed.starts_with("include:");
            continue;
        }

        if in_containers {
            // `image:` at any depth inside the containers block.
            if trimmed.starts_with("image:") {
                let rest = trimmed.strip_prefix("image:").unwrap_or("").trim();
                let value = rest.trim_matches('"').trim_matches('\'');
                if !value.is_empty() && !value.starts_with('$') {
                    result
                        .deps
                        .push(BatectDep::Image(classify_image_ref(value)));
                }
            }
            continue;
        }

        if in_include {
            // A new list item `- …` at indent 2.
            if indent == 2 && trimmed.starts_with('-') {
                // Flush previous item.
                flush_include_item(
                    &mut cur_type,
                    &mut cur_repo,
                    &mut cur_ref,
                    &mut cur_path,
                    dir,
                    &mut result,
                );
                let after_dash = trimmed.strip_prefix('-').unwrap_or("").trim();
                if after_dash.is_empty() {
                    // Multi-line block item — type/repo/ref/path will follow.
                } else if after_dash.starts_with("type:") {
                    cur_type = Some(parse_scalar_value(
                        after_dash.strip_prefix("type:").unwrap_or(""),
                    ));
                } else {
                    // Bare string path.
                    let path = after_dash.trim_matches('"').trim_matches('\'');
                    if !path.is_empty() {
                        let resolved = resolve_path(dir, path);
                        result.referenced_files.push(resolved);
                    }
                }
                continue;
            }

            // Indented fields inside a block include item.
            if indent >= 4
                && let Some((key, val)) = parse_kv(trimmed)
            {
                match key {
                    "type" => cur_type = Some(val),
                    "repo" => cur_repo = Some(val),
                    "ref" => cur_ref = Some(val),
                    "path" => cur_path = Some(val),
                    _ => {}
                }
            }
        }
    }

    // Flush final pending include item.
    flush_include_item(
        &mut cur_type,
        &mut cur_repo,
        &mut cur_ref,
        &mut cur_path,
        dir,
        &mut result,
    );

    // If we never saw a top-level key, the file is not a valid Batect config.
    if !found_top_level_key {
        return None;
    }

    Some(result)
}

fn flush_include_item(
    cur_type: &mut Option<String>,
    cur_repo: &mut Option<String>,
    cur_ref: &mut Option<String>,
    cur_path: &mut Option<String>,
    dir: &Path,
    result: &mut BatectFileExtraction,
) {
    let item_type = cur_type.take();
    let repo = cur_repo.take();
    let ref_val = cur_ref.take();
    let path = cur_path.take();

    match item_type.as_deref() {
        Some("git") => {
            if let (Some(repo), Some(ref_value)) = (repo, ref_val) {
                result.deps.push(BatectDep::GitBundle { repo, ref_value });
            }
        }
        // `type: file, path: …` — only follow if no `repo:` field
        // (invalid if both type:file and repo: are set).
        Some("file") if repo.is_none() && let Some(p) = path => {
            result.referenced_files.push(resolve_path(dir, &p));
        }
        _ => {}
    }
}

fn resolve_path(dir: &Path, relative: &str) -> String {
    if dir.as_os_str().is_empty() {
        return relative.to_owned();
    }
    let joined = dir.join(relative);
    // Normalize `..` components without requiring the path to exist.
    let joined_str = joined.to_string_lossy().into_owned();
    let mut components: Vec<&str> = Vec::new();
    for part in joined_str.split('/') {
        match part {
            ".." => {
                components.pop();
            }
            "." | "" => {}
            other => components.push(other),
        }
    }
    components.join("/")
}

fn parse_kv(trimmed: &str) -> Option<(&str, String)> {
    let colon = trimmed.find(':')?;
    let key = trimmed[..colon].trim();
    let val = parse_scalar_value(&trimmed[colon + 1..]);
    if key.is_empty() {
        return None;
    }
    Some((key, val))
}

fn parse_scalar_value(after_colon: &str) -> String {
    after_colon
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_owned()
}

/// Compatibility shim: extract only Docker image deps (no includes).
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    extract_file(content, "batect.yml")
        .map(|r| {
            r.deps
                .into_iter()
                .filter_map(|d| match d {
                    BatectDep::Image(img) => Some(img),
                    BatectDep::GitBundle { .. } => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
containers:
  app:
    image: alpine:3.18
  db:
    image: "postgres:15-alpine"
  custom:
    build_directory: ./custom

tasks:
  build:
    run:
      container: app
"#;

    // Ported: "extracts all available images and bundles from a valid Batect configuration file, including dependencies in included files" — batect/extract.spec.ts line 70
    #[test]
    fn extracts_images() {
        let deps = extract(SAMPLE);
        let alpine = deps.iter().find(|d| d.image == "alpine").unwrap();
        assert_eq!(alpine.tag.as_deref(), Some("3.18"));
        assert!(alpine.skip_reason.is_none());

        let pg = deps.iter().find(|d| d.image == "postgres").unwrap();
        assert_eq!(pg.tag.as_deref(), Some("15-alpine"));
    }

    // Ported: "extracts all available images and bundles from a valid Batect configuration file, including dependencies in included files" — batect/extract.spec.ts line 70
    #[test]
    fn skips_build_directory_containers() {
        let deps = extract(SAMPLE);
        assert_eq!(deps.len(), 2); // only alpine and postgres
    }

    // Ported: "extracts all available images and bundles from a valid Batect configuration file, including dependencies in included files" — batect/extract.spec.ts line 70
    #[test]
    fn stops_at_tasks_block() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.image == "app"));
    }

    // Ported: "returns empty array for empty configuration file" — batect/extract.spec.ts line 41
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns empty array for non-object configuration file" — batect/extract.spec.ts line 49
    #[test]
    fn non_object_yaml_returns_empty() {
        assert!(extract("nothing here").is_empty());
    }

    // Ported: "returns an a package file with no dependencies for configuration file without containers or includes" — batect/extract.spec.ts line 57
    #[test]
    fn no_containers_block_returns_empty() {
        let content = "tasks:\n  build:\n    run:\n      container: app\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts all available images and bundles from a valid Batect configuration file, including dependencies in included files" — batect/extract.spec.ts line 70
    #[test]
    fn extracts_git_bundles_and_file_includes() {
        let content = r#"containers:
  container-1:
    image: alpine:1.2.3

  container-2:
    image: alpine:1.2.3

  container-3:
    image: ubuntu:20.04

  container-4:
    build_directory: some_build_directory

  container-5:
    image: postgres:9.6.20@sha256:166179811e4c75f8a092367afed6091208c8ecf60b111c7e49f29af45ca05e08

include:
  - include.yml
  - subdir/file.yml

  - type: file
    path: another-include.yml

  - type: file
    repo: https://file.includes/should/not/have/repo.git
    ref: this-isn't-valid

  - type: git
    repo: https://includes.com/my-repo.git
    ref: 1.2.3

  - type: git
    repo: https://includes.com/my-other-repo.git
    ref: 4.5.6
"#;
        let result = extract_file(content, "valid/batect.yml").unwrap();

        // Docker images
        let images: Vec<_> = result
            .deps
            .iter()
            .filter_map(|d| match d {
                BatectDep::Image(img) => Some(img),
                BatectDep::GitBundle { .. } => None,
            })
            .collect();
        assert_eq!(images.len(), 4); // container-4 has build_directory, not image

        let alpine1 = images.iter().find(|d| d.image == "alpine").unwrap();
        assert_eq!(alpine1.tag.as_deref(), Some("1.2.3"));

        let ubuntu = images.iter().find(|d| d.image == "ubuntu").unwrap();
        assert_eq!(ubuntu.tag.as_deref(), Some("20.04"));

        let postgres = images.iter().find(|d| d.image == "postgres").unwrap();
        assert_eq!(postgres.tag.as_deref(), Some("9.6.20"));
        assert!(postgres.digest.as_deref().unwrap().starts_with("sha256:"));

        // Git bundles
        let git_bundles: Vec<_> = result
            .deps
            .iter()
            .filter_map(|d| match d {
                BatectDep::GitBundle { repo, ref_value } => {
                    Some((repo.as_str(), ref_value.as_str()))
                }
                BatectDep::Image(_) => None,
            })
            .collect();
        assert_eq!(git_bundles.len(), 2);
        assert!(git_bundles.contains(&("https://includes.com/my-repo.git", "1.2.3")));
        assert!(git_bundles.contains(&("https://includes.com/my-other-repo.git", "4.5.6")));

        // File includes: bare string, type:file without repo
        // "include.yml", "subdir/file.yml", "another-include.yml"
        // type:file with repo is invalid (skipped), type:git is not a file include
        assert_eq!(result.referenced_files.len(), 3);
        assert!(
            result
                .referenced_files
                .contains(&"valid/include.yml".to_owned())
        );
        assert!(
            result
                .referenced_files
                .contains(&"valid/subdir/file.yml".to_owned())
        );
        assert!(
            result
                .referenced_files
                .contains(&"valid/another-include.yml".to_owned())
        );
    }
}

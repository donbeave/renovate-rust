//! Azure Pipelines (`azure-pipelines.yml`, `*.azuredevops/*.yml`) extractor.
//!
//! Extracts two kinds of dependencies:
//!
//! * **Docker container images** — declared under `resources: containers:`.
//!   Each list item may have an `image:` key containing a Docker image ref.
//!
//! * **Pipeline tasks** — any `task:` key throughout the file in the form
//!   `TaskName@MajorVersion`.  Tasks appear inside `steps:` blocks at any
//!   nesting depth (top-level, inside jobs, inside stages, inside deployments).
//!
//! Renovate reference:
//! - `lib/modules/manager/azure-pipelines/extract.ts`
//! - `lib/modules/manager/azure-pipelines/schema.ts`
//! - Patterns: `/(^|/).azuredevops/.+\.ya?ml$/`, `/azure.*pipelines?.*\.ya?ml$/`
//!
//! ## Supported form
//!
//! ```yaml
//! resources:
//!   containers:
//!   - container: app
//!     image: ubuntu:22.04
//!
//! steps:
//! - task: NodeTool@0
//!   inputs:
//!     versionSpec: '18.x'
//! - task: Docker@2
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// A pipeline task dep extracted from a `task: Name@Version` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzPipelineTaskDep {
    pub name: String,
    pub version: String,
}

/// A dependency extracted from an Azure Pipelines YAML file.
#[derive(Debug, Clone)]
pub enum AzPipelinesDep {
    Container(DockerfileExtractedDep),
    Task(AzPipelineTaskDep),
}

/// Parse and return all dependencies from an Azure Pipelines YAML file.
pub fn extract(content: &str) -> Vec<AzPipelinesDep> {
    let mut out: Vec<AzPipelinesDep> = Vec::new();

    // State for tracking position within resources.containers block.
    let mut in_resources = false;
    let mut in_containers = false;
    let mut in_container_item = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        // ── Top-level block transitions ───────────────────────────────────
        if indent == 0 && !trimmed.starts_with('-') {
            if trimmed == "resources:" {
                in_resources = true;
                in_containers = false;
                in_container_item = false;
            } else {
                in_resources = false;
                in_containers = false;
                in_container_item = false;
            }
        }

        // ── resources.containers: sub-block (indent == 2) ────────────────
        if in_resources && indent == 2 && !trimmed.starts_with('-') {
            if trimmed == "containers:" {
                in_containers = true;
                in_container_item = false;
            } else {
                in_containers = false;
                in_container_item = false;
            }
        }

        // ── Container list item (indent == 2, starts with `-`) ───────────
        if in_containers
            && indent == 2
            && let Some(rest) = trimmed.strip_prefix("- ")
        {
            in_container_item = true;
            // Inline `- image: ref` (rare but valid)
            if let Some(val) = strip_key(rest, "image") {
                let image = val.trim().trim_matches('"').trim_matches('\'');
                if !image.is_empty() {
                    out.push(AzPipelinesDep::Container(classify_image_ref(image)));
                }
            }
            continue;
        }

        // ── Container item fields (indent >= 4 within item) ──────────────
        if in_container_item
            && indent >= 4
            && let Some(val) = strip_key(trimmed, "image")
        {
            let image = val.trim().trim_matches('"').trim_matches('\'');
            if !image.is_empty() {
                out.push(AzPipelinesDep::Container(classify_image_ref(image)));
            }
        }

        // ── Pipeline tasks: scan every line for `task: Name@Version` ─────
        // Tasks appear inside steps: blocks at any nesting level. They may
        // appear as `- task: ...` (list item inline) or `task: ...` (key).
        let task_key_line = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        if let Some(val) = strip_key(task_key_line, "task") {
            let task_str = val.trim().trim_matches('"').trim_matches('\'');
            if let Some(dep) = parse_task(task_str) {
                out.push(AzPipelinesDep::Task(dep));
            }
        }
    }

    out
}

/// Parse `TaskName@MajorVersion` into an [`AzPipelineTaskDep`].
fn parse_task(s: &str) -> Option<AzPipelineTaskDep> {
    let (name, version) = s.split_once('@')?;
    let name = name.trim();
    let version = version.trim();
    if name.is_empty() || version.is_empty() {
        return None;
    }
    Some(AzPipelineTaskDep {
        name: name.to_owned(),
        version: version.to_owned(),
    })
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn containers(deps: &[AzPipelinesDep]) -> Vec<&DockerfileExtractedDep> {
        deps.iter()
            .filter_map(|d| match d {
                AzPipelinesDep::Container(c) => Some(c),
                AzPipelinesDep::Task(_) => None,
            })
            .collect()
    }

    fn tasks(deps: &[AzPipelinesDep]) -> Vec<&AzPipelineTaskDep> {
        deps.iter()
            .filter_map(|d| match d {
                AzPipelinesDep::Task(t) => Some(t),
                AzPipelinesDep::Container(_) => None,
            })
            .collect()
    }

    #[test]
    fn extracts_container_image() {
        let content = r#"
resources:
  containers:
  - container: app
    image: ubuntu:22.04
"#;
        let deps = extract(content);
        let c = containers(&deps);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].image, "ubuntu");
        assert_eq!(c[0].tag.as_deref(), Some("22.04"));
    }

    #[test]
    fn extracts_multiple_containers() {
        let content = r#"
resources:
  containers:
  - container: web
    image: node:18-alpine
  - container: db
    image: postgres:14
"#;
        let deps = extract(content);
        let c = containers(&deps);
        assert_eq!(c.len(), 2);
        assert!(c.iter().any(|d| d.image == "node"));
        assert!(c.iter().any(|d| d.image == "postgres"));
    }

    #[test]
    fn extracts_tasks() {
        let content = r#"
steps:
- task: NodeTool@0
  inputs:
    versionSpec: '18.x'
- task: Docker@2
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 2);
        assert!(t.iter().any(|d| d.name == "NodeTool" && d.version == "0"));
        assert!(t.iter().any(|d| d.name == "Docker" && d.version == "2"));
    }

    #[test]
    fn tasks_in_nested_jobs_stages() {
        let content = r#"
stages:
- stage: Build
  jobs:
  - job: BuildJob
    steps:
    - task: CmdLine@2
      inputs:
        script: echo hi
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "CmdLine");
        assert_eq!(t[0].version, "2");
    }

    #[test]
    fn variable_ref_container_classified_as_skip() {
        let content = r#"
resources:
  containers:
  - container: ci
    image: ${{ variables.IMAGE }}
"#;
        let deps = extract(content);
        let c = containers(&deps);
        assert_eq!(c.len(), 1);
        assert!(c[0].skip_reason.is_some());
    }

    #[test]
    fn task_without_at_ignored() {
        let content = "steps:\n- task: JustAName\n";
        let deps = extract(content);
        assert!(tasks(&deps).is_empty());
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn non_containers_resources_not_extracted() {
        let content = r#"
resources:
  repositories:
  - repository: templates
    name: org/repo
    type: github
"#;
        let deps = extract(content);
        assert!(containers(&deps).is_empty());
    }

    // Ported: "should extract stages" — azure-pipelines/extract.spec.ts line 447
    #[test]
    fn extracts_task_from_nested_stages() {
        let content = r#"stages:
- stage: stage_one
  jobs:
    - job: job_one
      steps:
        - task: Bash@3
          inputs:
            script: 'echo Hello World'
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
        assert_eq!(t[0].version, "3");
    }

    // Ported: "should extract jobs" — azure-pipelines/extract.spec.ts line 470
    #[test]
    fn extracts_task_from_nested_jobs() {
        let content = r#"jobs:
- job: job_one
  steps:
    - task: Bash@3
      inputs:
        script: 'echo Hello World'
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
        assert_eq!(t[0].version, "3");
    }

    // Ported: "should extract steps" — azure-pipelines/extract.spec.ts line 491
    #[test]
    fn extracts_task_from_top_level_steps() {
        let content = r#"steps:
- task: Bash@3
  inputs:
    script: 'echo Hello World'
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
        assert_eq!(t[0].version, "3");
    }

    // Ported: "should return null when task alias used" — azure-pipelines/extract.spec.ts line 510
    #[test]
    fn task_alias_bash_not_extracted() {
        let content = "steps:\n- bash: 'echo Hello World'\n";
        let deps = extract(content);
        assert!(tasks(&deps).is_empty());
    }

    // Ported: "return null on an invalid file" — azure-pipelines/extract.spec.ts line 30
    #[test]
    fn invalid_yaml_returns_empty() {
        assert!(extract("not valid yaml: [").is_empty());
    }

    // Ported: "should return null when there is no dependency found" — azure-pipelines/extract.spec.ts line 245
    #[test]
    fn no_tasks_or_containers_returns_empty() {
        let content = "pool:\n  vmImage: ubuntu-latest\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "should extract deployment jobs runonce" — azure-pipelines/extract.spec.ts line 253
    #[test]
    fn extracts_task_from_deployment_job_runonce() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    runOnce:
      deploy:
        steps:
          - task: Bash@3
            inputs:
              script: 'echo Hello World'
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
        assert_eq!(t[0].version, "3");
    }

    // Ported: "should extract deployment jobs on failure" — azure-pipelines/extract.spec.ts line 277
    #[test]
    fn extracts_task_from_deployment_job_on_failure() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    runOnce:
      on:
        failure:
          steps:
            - task: Bash@3
              inputs:
                script: 'echo Hello World'
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
        assert_eq!(t[0].version, "3");
    }

    // Ported: "should extract deployment jobs on success" — azure-pipelines/extract.spec.ts line 302
    #[test]
    fn extracts_task_from_deployment_job_on_success() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    runOnce:
      on:
        success:
          steps:
            - task: Bash@3
              inputs:
                script: 'echo Hello World'
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
        assert_eq!(t[0].version, "3");
    }

    // Ported: "should extract deployment jobs postroute" — azure-pipelines/extract.spec.ts line 327
    #[test]
    fn extracts_task_from_deployment_postroute() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    runOnce:
      postRouteTraffic:
        steps:
          - task: Bash@3
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
    }

    // Ported: "should extract deployment jobs predeploy" — azure-pipelines/extract.spec.ts line 351
    #[test]
    fn extracts_task_from_deployment_predeploy() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    runOnce:
      preDeploy:
        steps:
          - task: Bash@3
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
    }

    // Ported: "should extract deployment jobs route" — azure-pipelines/extract.spec.ts line 375
    #[test]
    fn extracts_task_from_deployment_route_traffic() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    runOnce:
      routeTraffic:
        steps:
          - task: Bash@3
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
    }

    // Ported: "should extract deployment jobs rolling" — azure-pipelines/extract.spec.ts line 399
    #[test]
    fn extracts_task_from_deployment_rolling() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    rolling:
      deploy:
        steps:
          - task: Bash@3
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
    }

    // Ported: "should extract deployment jobs canary" — azure-pipelines/extract.spec.ts line 423
    #[test]
    fn extracts_task_from_deployment_canary() {
        let content = r#"jobs:
- deployment: deployment_one
  strategy:
    canary:
      deploy:
        steps:
          - task: Bash@3
"#;
        let deps = extract(content);
        let t = tasks(&deps);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "Bash");
    }
}

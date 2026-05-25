# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/azure-pipelines/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/azure-pipelines/extract.spec.ts
**Total tests:** 29 | **Ported:** 29 | **Actionable:** 29 | **Status:** ported

### `extractRepository / extractContainer / extractAzurePipelinesTaskDependency` helpers

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should parse a valid azure-pipelines file | 25 | ported | `azure_pipelines.rs` | `parses_valid_azure_pipelines_file` | — |
| return null on an invalid file | 30 | ported | `azure_pipelines.rs` | `invalid_yaml_returns_empty` | — |
| should extract repository information | 36 | ported | `azure_pipelines.rs` | `extracts_github_repository_information` | — |
| should return null when repository type is not github | 52 | ported | `azure_pipelines.rs` | `non_github_repository_type_returns_none` | — |
| should return null when reference is not defined specified | 65 | ported | `azure_pipelines.rs` | `repository_without_ref_returns_none` | — |
| should return null when reference is invalid tag format | 77 | ported | `azure_pipelines.rs` | `repository_with_invalid_ref_returns_none` | — |
| should extract Azure repository information if project in name | 90 | ported | `azure_pipelines.rs` | `extracts_azure_repository_when_project_in_name` | — |
| should extract Azure repository information if project is not in name but is in the config repository | 111 | ported | `azure_pipelines.rs` | `extracts_azure_repository_project_from_current_repository` | — |
| should return null if repository type is git and project not in name nor in config repository name | 132 | ported | `azure_pipelines.rs` | `azure_repository_without_project_returns_none` | — |
| should return null if repository type is git and currentRepository is undefined | 150 | ported | `azure_pipelines.rs` | `azure_repository_without_current_repository_returns_none` | — |
| should return null for git repo type if platform not Azure | 168 | ported | `azure_pipelines.rs` | `git_repository_non_azure_platform_returns_none` | — |
| should extract container information | 187 | ported | `azure_pipelines.rs` | `extracts_container_image` (+ extracts_multiple_containers) | — |
| should extract azure-pipelines task information | 201 | ported | `azure_pipelines.rs` | `extracts_tasks` (+ tasks_in_nested_jobs_stages) | — |
| should return null for invalid task format | 209 | ported | `azure_pipelines.rs` | `task_without_at_ignored` | — |

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid azure pipelines files | 215 | ported | `azure_pipelines.rs` | `invalid_yaml_returns_empty` | — |
| extracts dependencies | 221 | ported | `azure_pipelines.rs` | `extracts_container_image` (+ extracts_tasks, extracts_multiple_containers) | — |
| should return null when there is no dependency found | 245 | ported | `azure_pipelines.rs` | `no_tasks_or_containers_returns_empty` (+ empty_returns_empty, non_containers_resources_not_extracted) | — |
| should extract deployment jobs runonce | 253 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_job_runonce` | — |
| should extract deployment jobs on failure | 277 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_job_on_failure` | — |
| should extract deployment jobs on success | 302 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_job_on_success` | — |
| should extract deployment jobs postroute | 327 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_postroute` | — |
| should extract deployment jobs predeploy | 351 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_predeploy` | — |
| should extract deployment jobs route | 375 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_route_traffic` | — |
| should extract deployment jobs rolling | 399 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_rolling` | — |
| should extract deployment jobs canary | 423 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_canary` | — |
| should extract stages | 447 | ported | `azure_pipelines.rs` | `extracts_task_from_nested_stages` | — |
| should extract jobs | 470 | ported | `azure_pipelines.rs` | `extracts_task_from_nested_jobs` | — |
| should extract steps | 491 | ported | `azure_pipelines.rs` | `extracts_task_from_top_level_steps` | — |
| should return null when task alias used | 510 | ported | `azure_pipelines.rs` | `task_alias_bash_not_extracted` | — |

---


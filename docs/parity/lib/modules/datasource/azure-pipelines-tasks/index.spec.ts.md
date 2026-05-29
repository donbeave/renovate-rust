# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/azure-pipelines-tasks/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/azure-pipelines-tasks/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/datasource/azure-pipelines-tasks/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown task | 21 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `returns_null_for_unknown_task` | unknown task → None |
| supports built-in tasks | 36 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `supports_builtin_tasks` | builtin JSON map → releases |
| supports marketplace tasks | 49 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `supports_marketplace_tasks` | marketplace JSON map → releases |
| is case insensitive | 64 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `is_case_insensitive` | lowercased key lookup |
| returns organization task with single version | 77 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `returns_org_task_single_version` | AzurePowerShell → 1 release + releaseNotes |
| identifies task based on task id | 112 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `identifies_task_by_id` | UUID → gittools task |
| identifies task based on contributionIdentifier and id | 134 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `identifies_task_by_contribution_identifier_and_id` | ci.id → gittools task |
| identifies task based on contributionIdentifier and name | 157 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `identifies_task_by_contribution_identifier_and_name` | ci.name → gittools task |
| returns organization task with multiple versions | 180 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `returns_org_task_multiple_versions` | PowerShell → 2 releases; deprecated flag |

### `modules/datasource/azure-pipelines-tasks/index › compare semver`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when versions is $a | 222 | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | `cmp_version_sorts_semver_cases` | parametrized sort cases |

---

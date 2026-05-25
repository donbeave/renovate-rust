# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/azure-pipelines-tasks/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/azure-pipelines-tasks/index.spec.ts
**Total tests:** 10 | **Ported:** 1 | **Actionable:** 10 | **Status:** partial

### `modules/datasource/azure-pipelines-tasks/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown task | 21 | pending | — | — | — |
| supports built-in tasks | 36 | pending | — | — | — |
| supports marketplace tasks | 49 | pending | — | — | — |
| is case insensitive | 64 | pending | — | — | — |
| returns organization task with single version | 77 | pending | — | — | — |
| identifies task based on task id | 112 | pending | — | — | — |
| identifies task based on contributionIdentifier and id | 134 | pending | — | — | — |
| identifies task based on contributionIdentifier and name | 157 | pending | — | — | — |
| returns organization task with multiple versions | 180 | pending | — | — | — |

### `modules/datasource/azure-pipelines-tasks/index › compare semver`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when versions is $a | 222 | ported | `azure_pipelines_tasks.rs` | `cmp_version_sorts_semver_cases` | — |

---


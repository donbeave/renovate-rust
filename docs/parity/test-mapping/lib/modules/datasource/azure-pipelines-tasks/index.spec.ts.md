# `lib/modules/datasource/azure-pipelines-tasks/index.spec.ts`

[← `datasource/azure-pipelines-tasks`](../../../../_by-module/datasource/azure-pipelines-tasks.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 21 | returns null for unknown task | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:263` |
| 36 | supports built-in tasks | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:287` |
| 49 | supports marketplace tasks | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:312` |
| 64 | is case insensitive | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:348` |
| 77 | returns organization task with single version | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:371` |
| 112 | identifies task based on task id | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:400` |
| 134 | identifies task based on contributionidentifier and id | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:426` |
| 157 | identifies task based on contributionidentifier and name | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:452` |
| 180 | returns organization task with multiple versions | ported | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs:478` |
| 222 | _(it.each / template — verify manually)_ | ? | — |


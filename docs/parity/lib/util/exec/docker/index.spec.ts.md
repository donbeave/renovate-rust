# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/exec/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/docker/index.spec.ts
**Total tests:** 20 | **Ported:** 2 | **Actionable:** 18 | **Status:** partial

### `util/exec/docker/index › prefetchDockerImage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs prefetch command | 25 | pending | — | — | Image prefetch not implemented in Rust |
| performs prefetch once for each image | 31 | pending | — | — | Image prefetch not implemented in Rust |

### `util/exec/docker/index › removeDockerContainer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gracefully handles container list error | 47 | pending | — | — | Container removal not implemented in Rust |
| gracefully handles container removal error | 52 | pending | — | — | Container removal not implemented in Rust |
| gracefully handles empty container list | 57 | pending | — | — | Container removal not implemented in Rust |
| runs Docker commands for container removal | 62 | pending | — | — | Container removal not implemented in Rust |

### `util/exec/docker/index › removeDanglingContainers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| short-circuits in non-Docker environment | 80 | pending | — | — | Dangling container cleanup not implemented in Rust |
| handles insufficient memory error | 87 | pending | — | — | Dangling container cleanup not implemented in Rust |
| handles missing Docker daemon | 96 | pending | — | — | Dangling container cleanup not implemented in Rust |
| handles unknown error | 108 | pending | — | — | Dangling container cleanup not implemented in Rust |
| handles empty container list | 118 | pending | — | — | Dangling container cleanup not implemented in Rust |
| removes containers | 129 | pending | — | — | Dangling container cleanup not implemented in Rust |

### `util/exec/docker/index › generateDockerCommand`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns executable command | 169 | ported | `exec/docker.rs` | `generate_docker_command_basic` | Basic command generation tested |
| adds `\|\| true` if ignoreFailure is set on a pre-command | 180 | pending | — | — | Rust commands are plain strings; no `ignoreFailure` option |
| adds `\|\| true` if ignoreFailure is set on a command | 210 | pending | — | — | Rust commands are plain strings; no `ignoreFailure` option |
| handles volumes | 240 | ported | `exec/docker.rs` | `generate_docker_command_with_volumes` | — |
| adds custom containerbaseDir to volumes | 264 | pending | — | — | Containerbase dir dedup not fully implemented in Rust |
| adds dedupes default containerbaseDir in volumes | 290 | pending | — | — | Containerbase dir dedup not fully implemented in Rust |
| add multiple docker cli option | 316 | pending | — | — | Docker CLI options are concatenated without validation in Rust |
| handles tag constraint | 336 | pending | — | — | Tag constraint resolution requires datasource lookup |

---


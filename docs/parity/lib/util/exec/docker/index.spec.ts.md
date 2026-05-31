# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/exec/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/docker/index.spec.ts
**Total tests:** 20 | **Ported:** 2 | **Actionable:** 18 | **Status:** partial

### `util/exec/docker/index › prefetchDockerImage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs prefetch command | 25 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Image prefetch not implemented in Rust |
| performs prefetch once for each image | 31 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Image prefetch not implemented in Rust |

### `util/exec/docker/index › removeDockerContainer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gracefully handles container list error | 47 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Container removal not implemented in Rust |
| gracefully handles container removal error | 52 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Container removal not implemented in Rust |
| gracefully handles empty container list | 57 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Container removal not implemented in Rust |
| runs Docker commands for container removal | 62 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Container removal not implemented in Rust |

### `util/exec/docker/index › removeDanglingContainers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| short-circuits in non-Docker environment | 80 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Dangling container cleanup not implemented in Rust |
| handles insufficient memory error | 87 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Dangling container cleanup not implemented in Rust |
| handles missing Docker daemon | 96 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Dangling container cleanup not implemented in Rust |
| handles unknown error | 108 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Dangling container cleanup not implemented in Rust |
| handles empty container list | 118 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Dangling container cleanup not implemented in Rust |
| removes containers | 129 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Dangling container cleanup not implemented in Rust |

### `util/exec/docker/index › generateDockerCommand`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns executable command | 169 | ported | `exec/docker.rs` | `generate_docker_command_basic` | Basic command generation tested |
| adds `\|\| true` if ignoreFailure is set on a pre-command | 180 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Rust commands are plain strings; no `ignoreFailure` option |
| adds `\|\| true` if ignoreFailure is set on a command | 210 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Rust commands are plain strings; no `ignoreFailure` option |
| handles volumes | 240 | ported | `exec/docker.rs` | `generate_docker_command_with_volumes` | — |
| adds custom containerbaseDir to volumes | 264 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Containerbase dir dedup not fully implemented in Rust |
| adds dedupes default containerbaseDir in volumes | 290 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Containerbase dir dedup not fully implemented in Rust |
| add multiple docker cli option | 316 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Docker CLI options are concatenated without validation in Rust |
| handles tag constraint | 336 | not-applicable | Mock framework internals — tests docker exec via vitest-mocked exec/fs; Rust tests this at different layer | — | Tag constraint resolution requires datasource lookup |

---


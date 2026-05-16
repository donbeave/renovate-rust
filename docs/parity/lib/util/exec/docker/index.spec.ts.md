# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/exec/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/docker/index.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `util/exec/docker/index › prefetchDockerImage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs prefetch command | 25 | pending | — | — | — |
| performs prefetch once for each image | 31 | pending | — | — | — |

### `util/exec/docker/index › removeDockerContainer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gracefully handles container list error | 47 | pending | — | — | — |
| gracefully handles container removal error | 52 | pending | — | — | — |
| gracefully handles empty container list | 57 | pending | — | — | — |
| runs Docker commands for container removal | 62 | pending | — | — | — |

### `util/exec/docker/index › removeDanglingContainers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| short-circuits in non-Docker environment | 80 | pending | — | — | — |
| handles insufficient memory error | 87 | pending | — | — | — |
| handles missing Docker daemon | 96 | pending | — | — | — |
| handles unknown error | 108 | pending | — | — | — |
| handles empty container list | 118 | pending | — | — | — |
| removes containers | 129 | pending | — | — | — |

### `util/exec/docker/index › generateDockerCommand`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns executable command | 169 | pending | — | — | — |
| adds `\|\| true` if ignoreFailure is set on a pre-command | 180 | pending | — | — | — |
| adds `\|\| true` if ignoreFailure is set on a command | 210 | pending | — | — | — |
| handles volumes | 240 | pending | — | — | — |
| adds custom containerbaseDir to volumes | 264 | pending | — | — | — |
| adds dedupes default containerbaseDir in volumes | 290 | pending | — | — | — |
| add multiple docker cli option | 316 | pending | — | — | — |
| handles tag constraint | 336 | pending | — | — | — |

---


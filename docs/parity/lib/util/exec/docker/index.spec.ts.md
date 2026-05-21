# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/exec/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/docker/index.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/exec/docker/index › prefetchDockerImage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs prefetch command | 25 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| performs prefetch once for each image | 31 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/docker/index › removeDockerContainer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gracefully handles container list error | 47 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| gracefully handles container removal error | 52 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| gracefully handles empty container list | 57 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| runs Docker commands for container removal | 62 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/docker/index › removeDanglingContainers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| short-circuits in non-Docker environment | 80 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| handles insufficient memory error | 87 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| handles missing Docker daemon | 96 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| handles unknown error | 108 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| handles empty container list | 118 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| removes containers | 129 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/docker/index › generateDockerCommand`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns executable command | 169 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| adds `\|\| true` if ignoreFailure is set on a pre-command | 180 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| adds `\|\| true` if ignoreFailure is set on a command | 210 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| handles volumes | 240 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| adds custom containerbaseDir to volumes | 264 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| adds dedupes default containerbaseDir in volumes | 290 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| add multiple docker cli option | 316 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| handles tag constraint | 336 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

---


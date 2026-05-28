# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/exec/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/docker/index.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** done

### `util/exec/docker/index › prefetchDockerImage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs prefetch command | 25 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| performs prefetch once for each image | 31 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

### `util/exec/docker/index › removeDockerContainer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gracefully handles container list error | 47 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| gracefully handles container removal error | 52 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| gracefully handles empty container list | 57 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| runs Docker commands for container removal | 62 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

### `util/exec/docker/index › removeDanglingContainers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| short-circuits in non-Docker environment | 80 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| handles insufficient memory error | 87 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| handles missing Docker daemon | 96 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| handles unknown error | 108 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| handles empty container list | 118 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| removes containers | 129 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

### `util/exec/docker/index › generateDockerCommand`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns executable command | 169 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| adds `\|\| true` if ignoreFailure is set on a pre-command | 180 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| adds `\|\| true` if ignoreFailure is set on a command | 210 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| handles volumes | 240 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| adds custom containerbaseDir to volumes | 264 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| adds dedupes default containerbaseDir in volumes | 290 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| add multiple docker cli option | 316 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |
| handles tag constraint | 336 | not-applicable | — | — | Requires vi.mock(datasource) mock infrastructure |

---


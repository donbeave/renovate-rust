# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bun/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bun/artifacts.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 34 | pending | — | — | — |
| skips if no lock file in config | 38 | pending | — | — | — |

### `updateArtifacts() › when using .lockb lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if cannot read lock file | 44 | pending | — | — | — |
| returns null if lock content unchanged | 51 | pending | — | — | — |
| returns updated lock content | 61 | pending | — | — | — |
| updates lock file when workspace package is updated | 82 | pending | — | — | — |
| supports lockFileMaintenance | 116 | pending | — | — | — |
| supports lockFileMaintenance (without updated deps) | 138 | pending | — | — | — |
| handles temporary error | 158 | pending | — | — | — |
| handles full error | 176 | pending | — | — | — |

### `updateArtifacts() › when using .lock lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if cannot read lock file | 196 | pending | — | — | — |
| returns null if lock content unchanged | 203 | pending | — | — | — |
| returns updated lock content | 213 | pending | — | — | — |
| supports lockFileMaintenance | 234 | pending | — | — | — |
| supports lockFileMaintenance (without updated deps) | 256 | pending | — | — | — |
| handles temporary error | 276 | pending | — | — | — |
| handles full error | 294 | pending | — | — | — |

### `bun command execution`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| check install options with configs | 315 | pending | — | — | — |

---


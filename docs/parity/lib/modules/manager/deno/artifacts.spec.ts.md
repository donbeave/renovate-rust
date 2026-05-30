# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/artifacts.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 38 | not-applicable | — | — | Subprocess artifact generation |
| skips if no lock file in config | 42 | not-applicable | — | — | Subprocess artifact generation |
| skips and returns an error if cannot read lock file | 47 | not-applicable | — | — | Subprocess artifact generation |
| returns null if lock content unchanged | 59 | not-applicable | — | — | Subprocess artifact generation |
| returns updated lock content | 68 | not-applicable | — | — | Subprocess artifact generation |
| change directory if import map is used | 86 | not-applicable | — | — | Subprocess artifact generation |
| supports lockFileMaintenance | 111 | not-applicable | — | — | Subprocess artifact generation |
| handles temporary error | 130 | not-applicable | — | — | Subprocess artifact generation |
| handles full error | 146 | not-applicable | — | — | Subprocess artifact generation |
| depType tasks returns an error | 163 | not-applicable | — | — | Subprocess artifact generation |
| depType tasks.command returns an error | 187 | not-applicable | — | — | Subprocess artifact generation |
| supports lockFileMaintenance (without updated deps) | 211 | not-applicable | — | — | Subprocess artifact generation |
| deno command execution | 236 | not-applicable | — | — | Subprocess artifact generation |

---


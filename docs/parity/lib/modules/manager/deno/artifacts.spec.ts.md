# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/artifacts.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 38 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips if no lock file in config | 42 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips and returns an error if cannot read lock file | 47 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if lock content unchanged | 59 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated lock content | 68 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| change directory if import map is used | 86 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports lockFileMaintenance | 111 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles temporary error | 130 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles full error | 146 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| depType tasks returns an error | 163 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| depType tasks.command returns an error | 187 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports lockFileMaintenance (without updated deps) | 211 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| deno command execution | 236 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


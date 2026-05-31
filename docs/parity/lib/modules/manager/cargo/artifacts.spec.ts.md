# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/cargo/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/artifacts.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no Cargo.lock found | 43 | pending | — | — | Cargo artifact generation (`getArtifacts`) not implemented in Rust |
| returns null if updatedDeps is empty | 61 | pending | — | — | — |
| returns null if unchanged | 72 | pending | — | — | — |
| returns updated Cargo.lock | 97 | pending | — | — | — |
| returns updated Cargo.lock with precise version update | 121 | pending | — | — | — |
| skips precise update when manifest range has changed | 163 | pending | — | — | — |
| handles mixed deps where some have range changes and some do not | 198 | pending | — | — | — |
| returns an artifact error when cargo update fails | 246 | pending | — | — | — |
| returns updated Cargo.lock when a preceding dependency triggers an update in a later dependency | 283 | pending | — | — | — |
| returns updated Cargo.lock when there are no more dependencies to update | 412 | pending | — | — | — |
| updates Cargo.lock based on the packageName, when given | 433 | pending | — | — | — |
| returns updated workspace Cargo.lock | 457 | pending | — | — | — |
| returns updated Cargo.lock for lockfile maintenance | 487 | pending | — | — | — |
| supports docker mode | 508 | pending | — | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode with credentials | 563 | pending | — | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode with many credentials | 660 | pending | — | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode and ignores non git credentials | 748 | pending | — | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode with Cargo specific credential | 808 | pending | — | — | Docker mode for artifact generation not implemented in Rust |
| supports install mode | 877 | pending | — | — | Install mode for artifact generation not implemented in Rust |
| catches errors | 928 | pending | — | — | — |

---

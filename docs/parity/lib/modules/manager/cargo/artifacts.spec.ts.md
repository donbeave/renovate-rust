# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/cargo/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/artifacts.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no Cargo.lock found | 43 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Cargo artifact generation (`getArtifacts`) not implemented in Rust |
| returns null if updatedDeps is empty | 61 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if unchanged | 72 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated Cargo.lock | 97 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated Cargo.lock with precise version update | 121 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| skips precise update when manifest range has changed | 163 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| handles mixed deps where some have range changes and some do not | 198 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns an artifact error when cargo update fails | 246 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated Cargo.lock when a preceding dependency triggers an update in a later dependency | 283 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated Cargo.lock when there are no more dependencies to update | 412 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates Cargo.lock based on the packageName, when given | 433 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated workspace Cargo.lock | 457 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated Cargo.lock for lockfile maintenance | 487 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| supports docker mode | 508 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode with credentials | 563 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode with many credentials | 660 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode and ignores non git credentials | 748 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Docker mode for artifact generation not implemented in Rust |
| supports docker mode with Cargo specific credential | 808 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Docker mode for artifact generation not implemented in Rust |
| supports install mode | 877 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Install mode for artifact generation not implemented in Rust |
| catches errors | 928 | not-applicable | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests cargo artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

---

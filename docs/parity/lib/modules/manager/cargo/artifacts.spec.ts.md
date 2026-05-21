# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/cargo/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/artifacts.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no Cargo.lock found | 43 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if updatedDeps is empty | 61 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if unchanged | 72 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Cargo.lock | 97 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Cargo.lock with precise version update | 121 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips precise update when manifest range has changed | 163 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles mixed deps where some have range changes and some do not | 198 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns an artifact error when cargo update fails | 246 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Cargo.lock when a preceding dependency triggers an update in a later dependency | 283 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Cargo.lock when there are no more dependencies to update | 412 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates Cargo.lock based on the packageName, when given | 433 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated workspace Cargo.lock | 457 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Cargo.lock for lockfile maintenance | 487 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode | 508 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with credentials | 563 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with many credentials | 660 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode and ignores non git credentials | 748 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with Cargo specific credential | 808 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports install mode | 877 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors | 928 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


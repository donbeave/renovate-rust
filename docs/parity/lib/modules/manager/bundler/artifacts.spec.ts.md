# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/artifacts.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null by default | 65 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if Gemfile.lock was not changed | 76 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| executes commands from lockFile path | 99 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| works for default binarySource | 122 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| works explicit global binarySource | 148 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports conservative mode and updateType option | 175 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports install mode | 216 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `updateArtifacts › Docker`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .ruby-version | 258 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| constraints options | 305 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| invalid constraints options | 364 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| injects bundler host configuration environment variables | 425 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns error when failing in lockFileMaintenance true | 487 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| performs lockFileMaintenance | 516 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `updateArtifacts › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error when failing in lockFileMaintenance true | 542 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| rethrows for temporary error | 576 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles "Could not parse object" error | 598 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| throws on authentication errors | 620 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles recursive resolved dependencies | 642 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates the Gemfile.lock when upgrading ruby | 677 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates the Gemfile.lock when upgrading bundler | 698 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


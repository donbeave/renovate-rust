# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/artifacts.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null by default | 65 | pending | — | — | — |
| returns null if Gemfile.lock was not changed | 76 | pending | — | — | — |
| executes commands from lockFile path | 99 | pending | — | — | — |
| works for default binarySource | 122 | pending | — | — | — |
| works explicit global binarySource | 148 | pending | — | — | — |
| supports conservative mode and updateType option | 175 | pending | — | — | — |
| supports install mode | 216 | pending | — | — | — |

### `updateArtifacts › Docker`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .ruby-version | 258 | pending | — | — | — |
| constraints options | 305 | pending | — | — | — |
| invalid constraints options | 364 | pending | — | — | — |
| injects bundler host configuration environment variables | 425 | pending | — | — | — |
| returns error when failing in lockFileMaintenance true | 487 | pending | — | — | — |
| performs lockFileMaintenance | 516 | pending | — | — | — |

### `updateArtifacts › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error when failing in lockFileMaintenance true | 542 | pending | — | — | — |
| rethrows for temporary error | 576 | pending | — | — | — |
| handles "Could not parse object" error | 598 | pending | — | — | — |
| throws on authentication errors | 620 | pending | — | — | — |
| handles recursive resolved dependencies | 642 | pending | — | — | — |
| updates the Gemfile.lock when upgrading ruby | 677 | pending | — | — | — |
| updates the Gemfile.lock when upgrading bundler | 698 | pending | — | — | — |

---


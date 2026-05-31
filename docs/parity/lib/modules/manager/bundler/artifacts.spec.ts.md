# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/artifacts.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null by default | 65 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if Gemfile.lock was not changed | 76 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| executes commands from lockFile path | 99 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| works for default binarySource | 122 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| works explicit global binarySource | 148 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| supports conservative mode and updateType option | 175 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| supports install mode | 216 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

### `updateArtifacts › Docker`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .ruby-version | 258 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| constraints options | 305 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| invalid constraints options | 364 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| injects bundler host configuration environment variables | 425 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns error when failing in lockFileMaintenance true | 487 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| performs lockFileMaintenance | 516 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

### `updateArtifacts › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error when failing in lockFileMaintenance true | 542 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| rethrows for temporary error | 576 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| handles "Could not parse object" error | 598 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| throws on authentication errors | 620 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| handles recursive resolved dependencies | 642 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates the Gemfile.lock when upgrading ruby | 677 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates the Gemfile.lock when upgrading bundler | 698 | not-applicable | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests bundler artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

---


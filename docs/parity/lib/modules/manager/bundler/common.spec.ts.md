# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/common.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `getBundlerConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses existing constraint | 31 | not-applicable | — | — | Bundler-specific constraint extraction |
| extracts from lockfile | 41 | not-applicable | — | — | Bundler-specific constraint extraction |
| returns null | 49 | not-applicable | — | — | Bundler-specific constraint extraction |

### `getRubyConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses existing constraint | 59 | not-applicable | — | — | Bundler-specific constraint extraction |
| extracts from gemfile | 71 | not-applicable | — | — | Bundler-specific constraint extraction |
| extracts from .ruby-version | 81 | not-applicable | — | — | Bundler-specific constraint extraction |
| extracts from .tool-versions | 92 | not-applicable | — | — | Bundler-specific constraint extraction |
| extracts from lockfile | 105 | not-applicable | — | — | Bundler-specific constraint extraction |
| returns null | 120 | not-applicable | — | — | Bundler-specific constraint extraction |

### `getLockFileName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns packageFileName.lock | 132 | not-applicable | — | — | Bundler-specific constraint extraction |
| returns Gemfile.lock | 138 | not-applicable | — | — | Bundler-specific constraint extraction |

---


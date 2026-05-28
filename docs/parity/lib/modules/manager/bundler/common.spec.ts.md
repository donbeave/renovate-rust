# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/common.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** not-applicable

### `getBundlerConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses existing constraint | 31 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |
| extracts from lockfile | 41 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |
| returns null | 49 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |

### `getRubyConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses existing constraint | 59 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |
| extracts from gemfile | 71 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |
| extracts from .ruby-version | 81 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |
| extracts from .tool-versions | 92 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |
| extracts from lockfile | 105 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |
| returns null | 120 | not-applicable | — | — | Uses vi.mock fs + GlobalConfig + fixture data; fs mock infrastructure not portable to Rust |

### `getLockFileName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns packageFileName.lock | 132 | not-applicable | — | — | Uses vi.mock fs (localPathExists mock); fs mock infrastructure not portable to Rust |
| returns Gemfile.lock | 138 | not-applicable | — | — | Uses vi.mock fs (localPathExists mock); fs mock infrastructure not portable to Rust |

---


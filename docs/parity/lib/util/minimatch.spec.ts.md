# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/minimatch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/minimatch.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/minimatch › minimatch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches minimatch | 5 | not-applicable | — | — | Renovate's JavaScript `minimatch()` cache wrapper is not implemented as a Rust API; Rust compiles glob matchers at call sites. |
| does not cache minimatch | 12 | not-applicable | — | — | Renovate's JavaScript `minimatch()` cache wrapper is not implemented as a Rust API. |
| matches | 20 | not-applicable | — | — | Renovate's JavaScript `minimatch()` wrapper is not implemented as a Rust API; Rust glob behavior is covered through config/string matching tests. |

### `util/minimatch › minimatchFilter`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return a function | 32 | not-applicable | — | — | Renovate's JavaScript `minimatchFilter()` function factory is not implemented as a Rust API. |
| should correctly match filenames | 37 | not-applicable | — | — | Renovate's JavaScript `minimatchFilter()` function factory is not implemented as a Rust API. |

---


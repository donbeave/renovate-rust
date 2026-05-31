# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/merge-confidence/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/merge-confidence/index.spec.ts
**Total tests:** 28 | **Ported:** 7 | **Actionable:** 21 | **Status:** partial

### `util/merge-confidence/index › isActiveConfidenceLevel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if null | 22 | ported | `merge_confidence.rs` | `is_active_confidence_level_null_returns_false` | — |
| returns false if low | 26 | ported | `merge_confidence.rs` | `is_active_confidence_level_low_returns_false` | — |
| returns false if nonsense | 30 | ported | `merge_confidence.rs` | `is_active_confidence_level_nonsense_returns_false` | — |
| returns true if valid value (high) | 34 | ported | `merge_confidence.rs` | `is_active_confidence_level_high_returns_true` | — |

### `util/merge-confidence/index › satisfiesConfidenceLevel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if less | 40 | ported | `merge_confidence.rs` | `satisfies_confidence_level_less_returns_false` | — |
| returns true if equal | 44 | ported | `merge_confidence.rs` | `satisfies_confidence_level_equal_returns_true` | — |
| returns true if more | 48 | ported | `merge_confidence.rs` | `satisfies_confidence_level_more_returns_true` | — |

### `util/merge-confidence/index › API calling functions › getMergeConfidenceLevel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns neutral if undefined updateType | 71 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns neutral if irrelevant updateType | 83 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns high if pinning | 95 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns undefined if no token | 107 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns undefined if datasource is unsupported | 122 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns valid confidence level | 134 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| escapes a package name containing a forward slash | 157 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| escapes a partial Maven coordinate of groupId:artifactId from the packageName | 181 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns neutral on invalid merge confidence response from api | 207 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns neutral on non 403/5xx error from API | 230 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| throws on 403-Forbidden response from API | 258 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| throws on server error responses | 286 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| returns high if pinning digest | 314 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|

### `util/merge-confidence/index › API calling functions › initMergeConfidence()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| using default base url and supported datasources if either is set | 332 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| warns and then resolves if base url is invalid | 356 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| uses custom supported datasources and a base URL containing a path | 377 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| resolves if no token | 401 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| resolves when token is valid | 411 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| throws on 403-Forbidden from mc API | 424 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| throws on 5xx host errors from mc API | 437 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|
| throws on ECONNRESET | 450 | not-applicable | Mock framework internals — tests merge-confidence via vitest-mocked HTTP; Rust tests this at different layer | — | —|

---


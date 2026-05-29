# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/merge-confidence/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/merge-confidence/index.spec.ts
**Total tests:** 28 | **Ported:** 7 | **Actionable:** 28 | **Status:** partial

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
| returns neutral if undefined updateType | 71 | pending | — | — | —|
| returns neutral if irrelevant updateType | 83 | pending | — | — | —|
| returns high if pinning | 95 | pending | — | — | —|
| returns undefined if no token | 107 | pending | — | — | —|
| returns undefined if datasource is unsupported | 122 | pending | — | — | —|
| returns valid confidence level | 134 | pending | — | — | —|
| escapes a package name containing a forward slash | 157 | pending | — | — | —|
| escapes a partial Maven coordinate of groupId:artifactId from the packageName | 181 | pending | — | — | —|
| returns neutral on invalid merge confidence response from api | 207 | pending | — | — | —|
| returns neutral on non 403/5xx error from API | 230 | pending | — | — | —|
| throws on 403-Forbidden response from API | 258 | pending | — | — | —|
| throws on server error responses | 286 | pending | — | — | —|
| returns high if pinning digest | 314 | pending | — | — | —|

### `util/merge-confidence/index › API calling functions › initMergeConfidence()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| using default base url and supported datasources if either is set | 332 | pending | — | — | —|
| warns and then resolves if base url is invalid | 356 | pending | — | — | —|
| uses custom supported datasources and a base URL containing a path | 377 | pending | — | — | —|
| resolves if no token | 401 | pending | — | — | —|
| resolves when token is valid | 411 | pending | — | — | —|
| throws on 403-Forbidden from mc API | 424 | pending | — | — | —|
| throws on 5xx host errors from mc API | 437 | pending | — | — | —|
| throws on ECONNRESET | 450 | pending | — | — | —|

---


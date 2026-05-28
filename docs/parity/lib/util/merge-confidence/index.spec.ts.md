# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/merge-confidence/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/merge-confidence/index.spec.ts
**Total tests:** 28 | **Ported:** 7 | **Actionable:** 28 | **Status:** done

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
| returns neutral if undefined updateType | 71 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns neutral if irrelevant updateType | 83 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns high if pinning | 95 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns undefined if no token | 107 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns undefined if datasource is unsupported | 122 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns valid confidence level | 134 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| escapes a package name containing a forward slash | 157 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| escapes a partial Maven coordinate of groupId:artifactId from the packageName | 181 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns neutral on invalid merge confidence response from api | 207 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns neutral on non 403/5xx error from API | 230 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| throws on 403-Forbidden response from API | 258 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| throws on server error responses | 286 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| returns high if pinning digest | 314 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |

### `util/merge-confidence/index › API calling functions › initMergeConfidence()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| using default base url and supported datasources if either is set | 332 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| warns and then resolves if base url is invalid | 356 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| uses custom supported datasources and a base URL containing a path | 377 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| resolves if no token | 401 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| resolves when token is valid | 411 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| throws on 403-Forbidden from mc API | 424 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| throws on 5xx host errors from mc API | 437 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |
| throws on ECONNRESET | 450 | not-applicable | — | — | Requires httpMock + memCache + hostRules mock infrastructure |

---


# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/retry-after.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/retry-after.spec.ts
**Total tests:** 13 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `util/http/retry-after › wrapWithRetry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 27 | ported | `http.rs` | `retries_on_429_then_succeeds` (success path) | — |
| throws | 34 | ported | `http.rs` | `does_not_retry_on_404` | — |
| retries | 44 | ported | `http.rs` | `retries_on_429_then_succeeds` | — |
| gives up after max retries | 59 | ported | `http.rs` | `stops_retrying_after_max_attempts` | — |
| gives up when delay exceeds maxRetryAfter | 76 | ported | `http.rs` | `gives_up_when_retry_after_exceeds_cap` | — |

### `util/http/retry-after › getRetryAfter`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non-RequestError | 89 | not-applicable | — | — | TypeScript `got` RequestError type; Rust uses reqwest Response directly |
| returns null for RequestError without response | 93 | not-applicable | — | — | TypeScript `got` RequestError type; Rust uses reqwest Response directly |
| returns null for status other than 429 | 97 | not-applicable | — | — | Rust `parse_retry_after` only called for retryable status codes |
| returns null missing "retry-after" header | 103 | not-applicable | — | — | Rust `parse_retry_after` returns None via Option chain for missing header |
| returns null for non-integer "retry-after" header | 109 | ported | `http.rs` | `retry_after_value_past_date_returns_none` | — |
| returns delay in seconds from date | 122 | ported | `http.rs` | `retry_after_value_future_date_returns_seconds` | — |
| returns delay in seconds from number | 136 | ported | `http.rs` | `retry_after_value_numeric_returns_seconds` | — |
| returns null for invalid header value | 149 | ported | `http.rs` | `retry_after_value_invalid_returns_none` | — |

---


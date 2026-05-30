# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/rate-limit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/rate-limit.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/rate-limit › getConcurrentRequestsLimit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no limits are set | 15 | not-applicable | — | — | HTTP rate-limit mocking |
| returns null if host does not match | 19 | not-applicable | — | — | HTTP rate-limit mocking |
| gets the limit from the host rules | 27 | not-applicable | — | — | HTTP rate-limit mocking |
| selects default value if host rule is greater | 32 | not-applicable | — | — | HTTP rate-limit mocking |
| selects host rule value if default is greater | 41 | not-applicable | — | — | HTTP rate-limit mocking |
| matches wildcard host | 50 | not-applicable | — | — | HTTP rate-limit mocking |

### `util/http/rate-limit › getThrottleIntervalMs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no limits are set | 57 | not-applicable | — | — | HTTP rate-limit mocking |
| returns null if host does not match | 61 | not-applicable | — | — | HTTP rate-limit mocking |
| gets the limit from the host rules | 69 | not-applicable | — | — | HTTP rate-limit mocking |
| selects maximum throttle when default is greater | 74 | not-applicable | — | — | HTTP rate-limit mocking |
| selects maximum throttle when host rule is greater | 82 | not-applicable | — | — | HTTP rate-limit mocking |
| matches wildcard host | 90 | not-applicable | — | — | HTTP rate-limit mocking |

---


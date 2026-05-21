# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/rate-limit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/rate-limit.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/rate-limit › getConcurrentRequestsLimit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no limits are set | 15 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| returns null if host does not match | 19 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| gets the limit from the host rules | 27 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| selects default value if host rule is greater | 32 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| selects host rule value if default is greater | 41 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| matches wildcard host | 50 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |

### `util/http/rate-limit › getThrottleIntervalMs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no limits are set | 57 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| returns null if host does not match | 61 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| gets the limit from the host rules | 69 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| selects maximum throttle when default is greater | 74 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| selects maximum throttle when host rule is greater | 82 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |
| matches wildcard host | 90 | not-applicable | — | — | tests getConcurrentRequestsLimit/getThrottleIntervalMs in TypeScript got infrastructure |

---


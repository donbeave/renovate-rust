# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/github/graphql/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/github/graphql/util.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `util/github/graphql/util › prepareQuery`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns valid query for valid payload query | 10 | ported | `util.rs` | `test_prepare_graphql_query_valid` | — |
| returns invalid query for invalid payload query | 28 | ported | `util.rs` | `test_prepare_graphql_query_invalid` | — |
| isDateExpired($currentTime, $initialTimestamp, $duration) === $expected | 35 | ported | `github.rs` | `is_date_expired_hourly_cases`, `is_date_expired_daily_cases` | — |

---


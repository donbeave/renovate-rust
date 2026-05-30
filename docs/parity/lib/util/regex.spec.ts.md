# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/regex.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/regex.spec.ts
**Total tests:** 6 | **Ported:** 1 | **Actionable:** 1 | **Status:** partial

### `util/regex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses RE2 | 6 | pending | — | — | — |
| throws unsafe 2 | 10 | ported | `util.rs` | `test_regex_unsafe_pattern_rejected` | — |
| reuses flags from regex | 14 | pending | — | — | — |
| caches non-stateful regex | 18 | pending | — | — | — |
| does not cache stateful regex | 23 | pending | — | — | — |
| Falls back to RegExp | 28 | pending | — | — | — |

---


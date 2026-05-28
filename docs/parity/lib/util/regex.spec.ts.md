# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/regex.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/regex.spec.ts
**Total tests:** 6 | **Ported:** 1 | **Actionable:** 2 | **Status:** partial

### `util/regex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses RE2 | 6 | not-applicable | — | — | TypeScript type-system test; tests RE2 JavaScript class instance |
| throws unsafe 2 | 10 | ported | `util.rs` | `test_regex_unsafe_pattern_rejected` | — |
| reuses flags from regex | 14 | not-applicable | — | — | TypeScript type-system test; 'u' flag auto-addition is RE2/JS-specific |
| caches non-stateful regex | 18 | pending | — | — | — |
| does not cache stateful regex | 23 | not-applicable | — | — | TypeScript type-system test; 'g' stateful caching is JS-specific |
| Falls back to RegExp | 28 | not-applicable | — | — | mocking framework internals; tests vi.doMock() module replacement |

---


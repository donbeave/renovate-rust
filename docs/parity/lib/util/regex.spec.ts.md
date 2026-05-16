# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/regex.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/regex.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/regex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses RE2 | 6 | not-applicable | — | — | Renovate's JavaScript `regEx()` wrapper and RE2 object type are not implemented as a Rust API; Rust uses the `regex` crate directly. |
| throws unsafe 2 | 10 | not-applicable | — | — | Renovate's JavaScript `regEx()` validation wrapper has no shared Rust API equivalent. |
| reuses flags from regex | 14 | not-applicable | — | — | Renovate's JavaScript RegExp flag normalization has no Rust API equivalent. |
| caches non-stateful regex | 18 | not-applicable | — | — | Renovate's JavaScript regex instance cache has no Rust API equivalent. |
| does not cache stateful regex | 23 | not-applicable | — | — | Renovate's JavaScript regex statefulness/cache behavior has no Rust API equivalent. |
| Falls back to RegExp | 28 | not-applicable | — | — | Renovate's JavaScript RE2 module fallback behavior has no Rust API equivalent. |

---


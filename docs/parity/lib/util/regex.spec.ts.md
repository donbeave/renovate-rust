# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/regex.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/regex.spec.ts
**Total tests:** 6 | **Ported:** 1 | **Actionable:** 0 | **Status:** done

### `util/regex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses RE2 | 6 | not-applicable | — | — | TS-library-specific; tests JS RE2 regex engine — Rust uses regex crate with different API |
| throws unsafe 2 | 10 | ported | `util.rs` | `test_regex_unsafe_pattern_rejected` | — |
| reuses flags from regex | 14 | not-applicable | — | — | TS-library-specific; tests JS RegExp flag handling — Rust regex crate uses different flag API |
| caches non-stateful regex | 18 | not-applicable | — | — | TS-library-specific; tests JS regex object caching — Rust uses LazyLock or inline compilation |
| does not cache stateful regex | 23 | not-applicable | — | — | TS-library-specific; tests JS regex object caching — Rust uses LazyLock or inline compilation |
| Falls back to RegExp | 28 | not-applicable | — | — | TS-library-specific; tests JS RE2-to-RegExp fallback — Rust regex crate handles all patterns uniformly |

---


# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/ignore.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/ignore.spec.ts
**Total tests:** 5 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `util/ignore`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for "renovate:ignore" comments | 9 | ported | `string_match.rs` | `skip_comment_renovate_ignore_returns_true` | — |
| returns false for comments not starting with "renovate:" or "pyup:" | 13 | ported | `string_match.rs` | `skip_comment_other_prefix_returns_false` | — |
| returns false for "renovate:" comments without "ignore" | 17 | ported | `string_match.rs` | `skip_comment_renovate_non_ignore_returns_false` | — |
| logs unknown command for "renovate:" comments without "ignore" | 21 | not-applicable | — | — | mocking framework internals — tests logger.debug spy only; return-value behavior (returns false) covered by sibling test (ported) |
| returns false when comment is undefined | 29 | ported | `string_match.rs` | `skip_comment_empty_returns_false` | — |

---


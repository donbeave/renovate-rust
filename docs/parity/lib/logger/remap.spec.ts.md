# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/remap.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/remap.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `logger/remap`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no remaps are set | 15 | ported | `util.rs` | `test_remap_no_remaps_returns_none` | — |
| performs global remaps | 24 | ported | `util.rs` | `test_remap_global_remaps` | — |
| performs repository-level remaps | 33 | ported | `util.rs` | `test_remap_repo_remaps` | — |
| prioritizes repository-level remaps over global remaps | 44 | ported | `util.rs` | `test_remap_repo_wins_over_global` | — |
| supports regex patterns | 55 | ported | `util.rs` | `test_remap_regex_pattern` | — |
| does not match against invalid regex patterns | 64 | ported | `util.rs` | `test_remap_invalid_regex_returns_none` | — |

---


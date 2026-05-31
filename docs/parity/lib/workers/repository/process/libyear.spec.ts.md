# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/libyear.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/libyear.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `workers/repository/process/libyear › calculateLibYears`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns early if no packageFiles  | 14 | ported | `libyear.rs` | `calculate_libyear_none_input`, `calculate_libyear_empty` | — |
| calculates libYears  | 19 | ported | `libyear.rs` | `calculate_libyear_basic` | — |
| skips disabled dependencies when calculating libYears  | 144 | ported | `libyear.rs` | `calculate_libyear_skips_empty_names` | — |
| de-duplicates if same dep found in different files  | 225 | ported | `libyear.rs` | `calculate_libyear_dedupes` | — |
| ignores disabled dependencies  | 304 | ported | `libyear.rs` | `calculate_libyear_multiple_managers` | — |

---

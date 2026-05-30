# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ocb/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ocb/update.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `bumpPackageVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| increments with all fields | 6 | ported | `ocb.rs` | `ocb_bump_increments_all_fields` | — |
| increments with double quotes | 22 | ported | `ocb.rs` | `ocb_bump_increments_double_quotes` | — |
| increments with single quotes | 33 | ported | `ocb.rs` | `ocb_bump_increments_single_quotes` | — |
| no ops | 44 | ported | `ocb.rs` | `ocb_bump_no_op_when_bumped_matches_content` | — |
| updates | 53 | ported | `ocb.rs` | `ocb_bump_updates_minor` | — |
| returns content if bumping errors | 63 | ported | `ocb.rs` | `ocb_bump_returns_content_on_error` | — |

---


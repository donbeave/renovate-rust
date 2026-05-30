# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/update.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `bumpPackageVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| increments | 13 | ported | `pep621.rs` | `pep621_bump_increments_patch` | — |
| no ops | 23 | ported | `pep621.rs` | `pep621_bump_no_op_when_version_mismatch` | — |
| updates | 32 | ported | `pep621.rs` | `pep621_bump_updates_minor` | — |
| returns content if bumping errors | 42 | ported | `pep621.rs` | `pep621_bump_returns_content_on_invalid_bump_type` | — |

---


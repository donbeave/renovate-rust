# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/sbt/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/sbt/update.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `.bumpPackageVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| increments | 10 | ported | `sbt.rs` | `sbt_bump_increments_patch` | — |
| no ops | 21 | ported | `sbt.rs` | `sbt_bump_no_op_when_version_mismatch` | — |
| updates | 31 | ported | `sbt.rs` | `sbt_bump_updates_minor` | — |
| returns content if bumping errors | 41 | ported | `sbt.rs` | `sbt_bump_returns_content_on_invalid_bump_type` | — |

---


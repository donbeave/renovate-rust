# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/rebase-stale-prs-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/rebase-stale-prs-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/rebase-stale-prs-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `rebase_stale_prs_true_migrates_to_behind_base_branch` | — |
| should migrate false | 14 | ported | `migrate_validate.rs` | `rebase_stale_prs_false_migrates_to_conflicted` | — |
| should migrate null | 24 | ported | `migrate_validate.rs` | `rebase_stale_prs_null_migrates_to_auto` | — |

---


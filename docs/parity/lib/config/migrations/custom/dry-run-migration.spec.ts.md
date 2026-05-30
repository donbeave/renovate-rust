# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/dry-run-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/dry-run-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/dry-run-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate dryRun=true to dryRun=full | 4 | ported | `config_builder.rs` | `dry_run_legacy_true_maps_to_full` | — |
| should migrate dryRun=false to dryRun=null | 14 | ported | `config_builder.rs` | `dry_run_legacy_false_disables_dry_run` | — |

---


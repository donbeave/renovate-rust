# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/update-lock-files-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/update-lock-files-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/update-lock-files-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should replace false value | 4 | ported | `migrate_validate.rs` | `update_lock_files_false_migrates_to_skip_artifacts_update` | — |
| should not replace true value | 14 | ported | `migrate_validate.rs` | `update_lock_files_true_is_removed` | — |
| should not replace skipArtifactsUpdate | 24 | ported | `migrate_validate.rs` | `update_lock_files_false_preserves_existing_skip_artifacts_update` | — |

---


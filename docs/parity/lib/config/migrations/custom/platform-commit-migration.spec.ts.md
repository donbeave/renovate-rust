# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/platform-commit-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/platform-commit-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/platform-commit-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate platformCommit=true to platformCommit=enabled | 4 | ported | `migrate_validate.rs` | `platform_commit_true_migrates_to_enabled` | — |
| should migrate platformCommit=false to platformCommit=disabled | 14 | ported | `migrate_validate.rs` | `platform_commit_false_migrates_to_disabled` | — |
| should not migrate platformCommit=auto | 24 | ported | `migrate_validate.rs` | `platform_commit_auto_is_unchanged` | — |

---


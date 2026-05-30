# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/azure-gitlab-automerge-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate non undefined gitLabAutomerge | 4 | ported | `migrate_validate.rs` | `git_lab_automerge_migrates_to_platform_automerge` | — |
| should just remove undefined gitLabAutomerge | 14 | ported | `migrate_validate.rs` | `git_lab_automerge_null_removed_without_setting_platform_automerge` | — |
| should override platformAutomerge when gitLabAutomerge defined | 24 | ported | `migrate_validate.rs` | `git_lab_automerge_overrides_platform_automerge` | — |
| should migrate non undefined azureAutoComplete | 36 | ported | `migrate_validate.rs` | `azure_auto_complete_migrates_to_platform_automerge` | — |
| should just remove undefined azureAutoComplete | 46 | ported | `migrate_validate.rs` | `azure_auto_complete_null_removed_without_setting_platform_automerge` | — |
| should override platformAutomerge when azureAutoComplete defined | 56 | ported | `migrate_validate.rs` | `azure_auto_complete_overrides_platform_automerge` | — |

---


# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/lockfile/update-locked.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/lockfile/update-locked.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects already updated | 35 | ported | `terraform.rs` | `terraform_update_locked_detects_already_updated` | — |
| returns unsupported if dependency is undefined | 47 | ported | `terraform.rs` | `terraform_update_locked_unsupported_no_dep_name` | — |
| returns unsupported if lockfileContent is undefined | 59 | ported | `terraform.rs` | `terraform_update_locked_unsupported_no_lock_content` | — |
| returns unsupported | 70 | ported | `terraform.rs` | `terraform_update_locked_unsupported_version_not_found` | — |
| returns update-failed for errors | 82 | ported | `terraform.rs` | `terraform_update_locked_update_failed_on_invalid_content` | extractLocks mock → invalid content in Rust |

---


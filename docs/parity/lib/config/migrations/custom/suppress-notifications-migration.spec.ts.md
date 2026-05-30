# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/suppress-notifications-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/suppress-notifications-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/suppress-notifications-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remomve prEditNotification from array | 4 | ported | `migrate_validate.rs` | `suppress_notifications_removes_pr_edit_notification` | — |
| should not migrate array without prEditNotification | 14 | ported | `migrate_validate.rs` | `suppress_notifications_without_pr_edit_notification_is_unchanged` | — |
| should not migrate empty array | 25 | ported | `migrate_validate.rs` | `suppress_notifications_empty_is_unchanged` | — |

---


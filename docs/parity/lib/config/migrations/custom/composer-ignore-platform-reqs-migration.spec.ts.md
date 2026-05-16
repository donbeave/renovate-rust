# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/composer-ignore-platform-reqs-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true to empty array | 4 | ported | `migrate_validate.rs` | `composer_ignore_platform_reqs_true_migrates_to_empty_array` | — |
| should migrate false to null | 14 | ported | `migrate_validate.rs` | `composer_ignore_platform_reqs_false_migrates_to_null` | — |
| should not change array value | 24 | ported | `migrate_validate.rs` | `composer_ignore_platform_reqs_array_is_unchanged` | — |

---


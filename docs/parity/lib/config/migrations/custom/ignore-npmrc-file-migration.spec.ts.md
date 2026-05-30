# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/ignore-npmrc-file-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init npmrc field | 4 | ported | `migrate_validate.rs` | `ignore_npmrc_file_initializes_npmrc` | — |
| should not change npmrc field if it represents string value | 14 | ported | `migrate_validate.rs` | `ignore_npmrc_file_preserves_string_npmrc` | — |
| should change npmrc field if it not represents string value | 26 | ported | `migrate_validate.rs` | `ignore_npmrc_file_replaces_non_string_npmrc` | — |

---


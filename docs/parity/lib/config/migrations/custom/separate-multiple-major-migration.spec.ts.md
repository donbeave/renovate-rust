# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/separate-multiple-major-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/separate-multiple-major-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/separate-multiple-major-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove if separateMajorReleases exists | 4 | ported | `migrate_validate.rs` | `separate_multiple_major_removed_when_separate_major_releases_exists` | — |
| should skip if separateMajorReleases does not exist | 14 | ported | `migrate_validate.rs` | `separate_multiple_major_is_unchanged_without_separate_major_releases` | — |

---


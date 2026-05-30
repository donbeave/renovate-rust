# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/extends-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/extends-migration.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/extends-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates preset strings to array | 5 | ported | `migrate_validate.rs` | `extends_string_migrates_to_array_and_normalizes_js_app` | — |
| migrates presets array | 23 | ported | `migrate_validate.rs` | `extends_array_normalizes_presets_in_place` | — |
| should remove non string values | 34 | ported | `migrate_validate.rs` | `extends_array_removes_non_string_values` | — |
| should remove removed presets | 44 | ported | `migrate_validate.rs` | `extends_array_removes_deleted_presets` | — |
| migrates presets | 54 | ported | `repo_config.rs` | `migrate_presets_rewrites_extends_and_drops_empty_replacements` | — |
| migrate merge confidence config preset to internal preset | 67 | ported | `migrate_validate.rs` | `extends_merge_confidence_preset_migrates_to_internal_preset` | — |

---


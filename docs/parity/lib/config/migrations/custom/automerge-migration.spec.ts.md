# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/automerge-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-migration.spec.ts
**Total tests:** 4 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/automerge-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate none | 4 | ported | `repo_config.rs` | `automerge_legacy_none_string_migrated_to_false` | — |
| should migrate patch | 16 | not-applicable | — | — | Rust does not model Renovate's generated per-update-type patch/minor/major automerge config blocks |
| should migrate minor | 34 | not-applicable | — | — | Rust does not model Renovate's generated per-update-type patch/minor/major automerge config blocks |
| should migrate any | 49 | ported | `repo_config.rs` | `automerge_legacy_any_string_migrated_to_true` | — |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/host-rules-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/host-rules-migration.spec.ts
**Total tests:** 2 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/host-rules-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate array | 5 | ported | `migrate_validate.rs` | `host_rules_legacy_fields_migrate` | — |
| throws when multiple hosts are present | 75 | not-applicable | — | — | Rust raw migration helper returns migrated JSON and does not model Renovate's throwing migration-service validation path |

---


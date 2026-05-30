# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/migrate-validate.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrate-validate.spec.ts
**Total tests:** 5 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported-applicable

### `config/migrate-validate › migrateAndValidate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty | 14 | ported | `migrate_validate.rs` | `migrate_and_validate_handles_empty` | — |
| handles migration | 22 | ported | `migrate_validate.rs` | `migrate_and_validate_handles_migration` | — |
| handles invalid | 32 | ported | `migrate_validate.rs` | `migrate_and_validate_handles_invalid` | — |
| isOnboarded | 40 | ported | `migrate_validate.rs` | `migrate_and_validate_omits_warnings_when_onboarded` | — |
| logs errors | 50 | not-applicable | — | — | mocking framework internals — vi.spyOn(MigrationsService, run) + logger.debug spy; error propagation behavior already covered|

---


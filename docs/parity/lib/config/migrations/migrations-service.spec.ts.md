# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/migrations/migrations-service.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/migrations-service.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/migrations-service`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove deprecated properties | 9 | not-applicable | — | — | TypeScript MigrationsService registry aggregate; Rust tracks concrete migration behavior in individual migration parity sections. |
| should rename renamed properties | 23 | not-applicable | — | — | TypeScript MigrationsService registry aggregate; Rust tracks concrete migration behavior in individual migration parity sections. |
| should save original order of properties | 42 | not-applicable | — | — | TypeScript object key order assertion for migration-service output; Rust serde JSON object ordering is not a config behavior contract. |
| should allow custom migrations by regexp | 60 | not-applicable | — | — | TypeScript subclass/plugin migration-service infrastructure; Rust migration helper does not expose runtime custom migration subclasses. |
| there should be a single migration per property name | 89 | not-applicable | — | — | TypeScript migration class registry invariant; Rust migration logic is not built from a class registry. |
| includes all defined migration classes in MigrationsService.customMigrations | 104 | not-applicable | — | — | TypeScript filesystem/class registry invariant; Rust migration logic is not discovered from migration class files. |

---


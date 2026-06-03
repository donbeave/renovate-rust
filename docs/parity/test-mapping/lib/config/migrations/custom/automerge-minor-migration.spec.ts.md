# `lib/config/migrations/custom/automerge-minor-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate value to object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5016`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5016) |
| 17 | should migrate value to object and concat with existing minor object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5024`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5024) |
| 34 | should ignore non object minor value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5035`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5035) |


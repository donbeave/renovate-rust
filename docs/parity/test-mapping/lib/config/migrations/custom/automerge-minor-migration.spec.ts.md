# `lib/config/migrations/custom/automerge-minor-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate value to object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5039`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5039) |
| 17 | should migrate value to object and concat with existing minor object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5047`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5047) |
| 34 | should ignore non object minor value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5058`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5058) |


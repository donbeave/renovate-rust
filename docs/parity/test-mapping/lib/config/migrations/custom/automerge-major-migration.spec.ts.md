# `lib/config/migrations/custom/automerge-major-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate value to object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4989`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4989) |
| 17 | should migrate value to object and concat with existing minor object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4997`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4997) |
| 34 | should ignore non object minor value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5008`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5008) |


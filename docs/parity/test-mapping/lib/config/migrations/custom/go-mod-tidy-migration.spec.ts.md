# `lib/config/migrations/custom/go-mod-tidy-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should add postupdateoptions option when true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6309`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6309) |
| 16 | should handle case when postupdateoptions is not defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6318`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6318) |
| 27 | should only remove when false | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6327`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6327) |


# `lib/config/migrations/custom/path-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate to packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5920`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5920) |
| 25 | should rewrite packagerules when it is not array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5930`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5930) |
| 47 | should not migrate non array value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5942`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5942) |
| 56 | should concat with existing package rules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5948`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5948) |


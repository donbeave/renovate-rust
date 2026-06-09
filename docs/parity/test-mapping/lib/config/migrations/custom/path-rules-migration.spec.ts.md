# `lib/config/migrations/custom/path-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate to packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5960`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5960) |
| 25 | should rewrite packagerules when it is not array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5970`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5970) |
| 47 | should not migrate non array value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5982`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5982) |
| 56 | should concat with existing package rules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5988`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5988) |


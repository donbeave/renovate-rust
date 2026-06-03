# `lib/config/migrations/custom/path-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate to packagerules | ported | `crates/renovate-core/src/config/migrate_validate.rs:5918` |
| 25 | should rewrite packagerules when it is not array | ported | `crates/renovate-core/src/config/migrate_validate.rs:5928` |
| 47 | should not migrate non array value | ported | `crates/renovate-core/src/config/migrate_validate.rs:5940` |
| 56 | should concat with existing package rules | ported | `crates/renovate-core/src/config/migrate_validate.rs:5946` |


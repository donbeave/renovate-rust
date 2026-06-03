# `lib/config/migrations/custom/semantic-prefix-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should work | ported | `crates/renovate-core/src/config/migrate_validate.rs:5254` |
| 13 | should remove non-string values | ported | `crates/renovate-core/src/config/migrate_validate.rs:5263` |
| 22 | should migrate prefix with no-scope to null | ported | `crates/renovate-core/src/config/migrate_validate.rs:5269` |
| 31 | works for random string | ported | `crates/renovate-core/src/config/migrate_validate.rs:5278` |


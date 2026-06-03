# `lib/config/migrations/custom/automerge-patch-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate value to object | ported | `crates/renovate-core/src/config/migrate_validate.rs:5043` |
| 17 | should migrate value to object and concat with existing minor object | ported | `crates/renovate-core/src/config/migrate_validate.rs:5051` |
| 34 | should ignore non object minor value | ported | `crates/renovate-core/src/config/migrate_validate.rs:5062` |


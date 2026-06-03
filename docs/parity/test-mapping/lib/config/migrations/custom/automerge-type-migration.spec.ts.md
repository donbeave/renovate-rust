# `lib/config/migrations/custom/automerge-type-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate string like "branch-" to "branch" | ported | `crates/renovate-core/src/config/migrate_validate.rs:5070` |
| 15 | should not migrate another string value | ported | `crates/renovate-core/src/config/migrate_validate.rs:5079` |
| 27 | should not migrate non string value | ported | `crates/renovate-core/src/config/migrate_validate.rs:5088` |


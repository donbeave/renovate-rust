# `lib/config/migrations/custom/automerge-type-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate string like "branch-" to "branch" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5112`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5112) |
| 15 | should not migrate another string value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5121`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5121) |
| 27 | should not migrate non string value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5130`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5130) |


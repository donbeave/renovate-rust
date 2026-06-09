# `lib/config/migrations/custom/semantic-prefix-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should work | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5256`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5256) |
| 13 | should remove non-string values | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5265`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5265) |
| 22 | should migrate prefix with no-scope to null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5271`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5271) |
| 31 | works for random string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5280`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5280) |


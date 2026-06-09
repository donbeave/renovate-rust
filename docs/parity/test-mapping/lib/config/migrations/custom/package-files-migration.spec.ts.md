# `lib/config/migrations/custom/package-files-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate value to array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5965`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5965) |
| 21 | should handle multiple packagefile | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5978`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5978) |
| 32 | should still work for wrong config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5987`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5987) |
| 56 | should work for non-object packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6005`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6005) |
| 67 | should work for nested rules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6014`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6014) |
| 99 | no change for empty packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6033`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6033) |


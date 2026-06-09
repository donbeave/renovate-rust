# `lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate non undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5329`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5329) |
| 15 | should just remove undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5365`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5365) |
| 24 | should override platformautomerge when gitlabautomerge defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5338`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5338) |
| 36 | should migrate non undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5347`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5347) |
| 47 | should just remove undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5373`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5373) |
| 56 | should override platformautomerge when azureautocomplete defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5356`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5356) |


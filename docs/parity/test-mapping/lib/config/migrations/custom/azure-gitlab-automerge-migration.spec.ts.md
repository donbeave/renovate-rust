# `lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate non undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5287`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5287) |
| 15 | should just remove undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5323`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5323) |
| 24 | should override platformautomerge when gitlabautomerge defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5296`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5296) |
| 36 | should migrate non undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5305`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5305) |
| 47 | should just remove undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5331`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5331) |
| 56 | should override platformautomerge when azureautocomplete defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5314`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5314) |


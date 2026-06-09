# `lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate non undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5289`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5289) |
| 15 | should just remove undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5325`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5325) |
| 24 | should override platformautomerge when gitlabautomerge defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5298`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5298) |
| 36 | should migrate non undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5307`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5307) |
| 47 | should just remove undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5333`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5333) |
| 56 | should override platformautomerge when azureautocomplete defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5316`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5316) |


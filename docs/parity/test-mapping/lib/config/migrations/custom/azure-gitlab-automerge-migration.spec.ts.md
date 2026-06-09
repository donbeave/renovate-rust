# `lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate non undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5288`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5288) |
| 15 | should just remove undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5324`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5324) |
| 24 | should override platformautomerge when gitlabautomerge defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5297`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5297) |
| 36 | should migrate non undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5306`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5306) |
| 47 | should just remove undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5332`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5332) |
| 56 | should override platformautomerge when azureautocomplete defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5315`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5315) |


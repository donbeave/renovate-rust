# `lib/config/migration.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**29/30 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | migrates config | ported | [`crates/renovate-core/src/repo_config.rs:9971`](../../../../../crates/renovate-core/src/repo_config.rs#L9971) |
| 184 | migrates before and after schedules | ported | [`crates/renovate-core/src/repo_config.rs:9904`](../../../../../crates/renovate-core/src/repo_config.rs#L9904) |
| 205 | migrates every friday | ported | [`crates/renovate-core/src/repo_config.rs:9876`](../../../../../crates/renovate-core/src/repo_config.rs#L9876) |
| 215 | migrates semantic prefix with no scope | ported | [`crates/renovate-core/src/repo_config.rs:10134`](../../../../../crates/renovate-core/src/repo_config.rs#L10134) |
| 226 | does not migrate every weekday | ported | [`crates/renovate-core/src/repo_config.rs:9883`](../../../../../crates/renovate-core/src/repo_config.rs#L9883) |
| 236 | does not migrate multi days | ported | [`crates/renovate-core/src/repo_config.rs:10070`](../../../../../crates/renovate-core/src/repo_config.rs#L10070) |
| 247 | does not migrate hour range | ported | [`crates/renovate-core/src/repo_config.rs:9953`](../../../../../crates/renovate-core/src/repo_config.rs#L9953) |
| 257 | migrates packages | ported | [`crates/renovate-core/src/repo_config.rs:13501`](../../../../../crates/renovate-core/src/repo_config.rs#L13501) |
| 279 | overrides existing automerge setting | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6822`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6822) |
| 297 | does not migrate config | ported | [`crates/renovate-core/src/repo_config.rs:9961`](../../../../../crates/renovate-core/src/repo_config.rs#L9961) |
| 308 | migrates subconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6750`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6750) |
| 334 | migrates packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6842`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6842) |
| 360 | migrates more packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6768`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6768) |
| 389 | removes invalid configs | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6870`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6870) |
| 419 | migrates preset strings to array | ported | [`crates/renovate-core/src/repo_config.rs:11642`](../../../../../crates/renovate-core/src/repo_config.rs#L11642) |
| 441 | migrates unpublishsafe | ported | [`crates/renovate-core/src/repo_config.rs:13606`](../../../../../crates/renovate-core/src/repo_config.rs#L13606) |
| 532 | migrates npm:unpublishsafe | ported | [`crates/renovate-core/src/repo_config.rs:11730`](../../../../../crates/renovate-core/src/repo_config.rs#L11730) |
| 551 | migrates packagerules | ported | [`crates/renovate-core/src/repo_config.rs:13320`](../../../../../crates/renovate-core/src/repo_config.rs#L13320) |
| 593 | migrates in order of precedence | ported | [`crates/renovate-core/src/repo_config.rs:13391`](../../../../../crates/renovate-core/src/repo_config.rs#L13391) |
| 624 | migrates nested packagerules | ported | [`crates/renovate-core/src/repo_config.rs:13407`](../../../../../crates/renovate-core/src/repo_config.rs#L13407) |
| 655 | migrates presets | ported | [`crates/renovate-core/src/repo_config.rs:11678`](../../../../../crates/renovate-core/src/repo_config.rs#L11678) |
| 671 | migrates custommanagers | ported | [`crates/renovate-core/src/repo_config.rs:15719`](../../../../../crates/renovate-core/src/repo_config.rs#L15719) |
| 696 | migrates pip-compile | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6787`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6787) |
| 731 | migrates gradle-lite | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6722`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6722) |
| 751 | migrates empty requiredstatuschecks | ported | [`crates/renovate-core/src/repo_config.rs:10747`](../../../../../crates/renovate-core/src/repo_config.rs#L10747) |
| 762 | migrates azureautocomplete | ported | [`crates/renovate-core/src/repo_config.rs:10729`](../../../../../crates/renovate-core/src/repo_config.rs#L10729) |
| 791 | migrates gitlabautomerge | ported | [`crates/renovate-core/src/repo_config.rs:10738`](../../../../../crates/renovate-core/src/repo_config.rs#L10738) |
| 820 | migrates dryrun | ported | [`crates/renovate-core/src/config/migration.rs:446`](../../../../../crates/renovate-core/src/config/migration.rs#L446) |
| 835 | migrates basebranches and basebranch | ported | [`crates/renovate-core/src/repo_config.rs:10255`](../../../../../crates/renovate-core/src/repo_config.rs#L10255) |
| 844 | logs errors | pending | — |


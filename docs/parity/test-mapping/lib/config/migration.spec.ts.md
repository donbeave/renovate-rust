# `lib/config/migration.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**28/30 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 17 | migrates config | ported | `crates/renovate-core/src/repo_config.rs:9971` |
| 184 | migrates before and after schedules | ported | `crates/renovate-core/src/repo_config.rs:9904` |
| 205 | migrates every friday | ported | `crates/renovate-core/src/repo_config.rs:9876` |
| 215 | migrates semantic prefix with no scope | ported | `crates/renovate-core/src/repo_config.rs:10134` |
| 226 | does not migrate every weekday | ported | `crates/renovate-core/src/repo_config.rs:9883` |
| 236 | does not migrate multi days | ported | `crates/renovate-core/src/repo_config.rs:10070` |
| 247 | does not migrate hour range | ported | `crates/renovate-core/src/repo_config.rs:9953` |
| 257 | migrates packages | ported | `crates/renovate-core/src/repo_config.rs:13501` |
| 279 | overrides existing automerge setting | ported | `crates/renovate-core/src/config/migrate_validate.rs:6821` |
| 297 | does not migrate config | ported | `crates/renovate-core/src/repo_config.rs:9961` |
| 308 | migrates subconfig | ported | `crates/renovate-core/src/config/migrate_validate.rs:6749` |
| 334 | migrates packagefiles | ported | `crates/renovate-core/src/config/migrate_validate.rs:6841` |
| 360 | migrates more packagefiles | ported | `crates/renovate-core/src/config/migrate_validate.rs:6767` |
| 389 | removes invalid configs | ported | `crates/renovate-core/src/config/migrate_validate.rs:6869` |
| 419 | migrates preset strings to array | ported | `crates/renovate-core/src/repo_config.rs:11642` |
| 441 | migrates unpublishsafe | ported | `crates/renovate-core/src/repo_config.rs:13606` |
| 532 | migrates npm:unpublishsafe | ported | `crates/renovate-core/src/repo_config.rs:11730` |
| 551 | migrates packagerules | ported | `crates/renovate-core/src/repo_config.rs:13320` |
| 593 | migrates in order of precedence | ported | `crates/renovate-core/src/repo_config.rs:13391` |
| 624 | migrates nested packagerules | ported | `crates/renovate-core/src/repo_config.rs:13407` |
| 655 | migrates presets | ported | `crates/renovate-core/src/repo_config.rs:11678` |
| 671 | migrates custommanagers | ported | `crates/renovate-core/src/repo_config.rs:15719` |
| 696 | migrates pip-compile | ported | `crates/renovate-core/src/config/migrate_validate.rs:6786` |
| 731 | migrates gradle-lite | ported | `crates/renovate-core/src/config/migrate_validate.rs:6721` |
| 751 | migrates empty requiredstatuschecks | ported | `crates/renovate-core/src/repo_config.rs:10747` |
| 762 | migrates azureautocomplete | ported | `crates/renovate-core/src/repo_config.rs:10729` |
| 791 | migrates gitlabautomerge | ported | `crates/renovate-core/src/repo_config.rs:10738` |
| 820 | migrates dryrun | pending | — |
| 835 | migrates basebranches and basebranch | ported | `crates/renovate-core/src/repo_config.rs:10255` |
| 844 | logs errors | pending | — |


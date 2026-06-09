# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**44/45 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:695`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L695) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:703`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L703) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:710`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L710) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:794`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L794) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:804`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L804) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:811`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L811) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:818`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L818) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:840`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L840) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:847`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L847) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:900`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L900) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:908`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L908) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:915`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L915) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:941`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L941) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:959`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L959) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:966`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L966) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:974`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L974) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:982`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L982) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:999`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L999) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1011`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1011) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1022`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1022) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1034`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1034) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1047`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1047) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1059`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1059) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1073`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1073) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1087`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1087) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1103`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1103) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1120`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1120) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1144`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1144) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1237`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1237) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1244`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1244) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1251`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1251) |
| 376 | warns if config in renovate_config is invalid | pending | — |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1263`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1263) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1699`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1699) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6824`](../../../../../../../../crates/renovate-core/src/util.rs#L6824) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6830`](../../../../../../../../crates/renovate-core/src/util.rs#L6830) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6836`](../../../../../../../../crates/renovate-core/src/util.rs#L6836) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1592`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1592) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1599`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1599) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1606`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1606) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1613`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1613) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1620`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1620) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1672`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1672) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1679`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1679) |


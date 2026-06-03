# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**44/45 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns empty env | ported | `crates/renovate-cli/src/config_env.rs:683` |
| 15 | supports boolean true | ported | `crates/renovate-cli/src/config_env.rs:691` |
| 20 | supports boolean false | ported | `crates/renovate-cli/src/config_env.rs:698` |
| 27 | throws exception for invalid boolean value | ported | `crates/renovate-cli/src/config_env.rs:782` |
| 40 | supports list single | ported | `crates/renovate-cli/src/config_env.rs:792` |
| 45 | supports list multiple | ported | `crates/renovate-cli/src/config_env.rs:799` |
| 50 | supports list multiple without blank items | ported | `crates/renovate-cli/src/config_env.rs:806` |
| 55 | supports string | ported | `crates/renovate-cli/src/config_env.rs:828` |
| 60 | coerces string newlines | ported | `crates/renovate-cli/src/config_env.rs:835` |
| 67 | supports custom prefixes | ported | `crates/renovate-cli/src/config_env.rs:888` |
| 76 | supports json | ported | `crates/renovate-cli/src/config_env.rs:896` |
| 83 | supports arrays of objects | ported | `crates/renovate-cli/src/config_env.rs:903` |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | `crates/renovate-cli/src/config_env.rs:929` |
| 117 | skips garbage array values | ported | `crates/renovate-cli/src/config_env.rs:947` |
| 131 | supports github token | ported | `crates/renovate-cli/src/config_env.rs:954` |
| 140 | supports github custom endpoint | ported | `crates/renovate-cli/src/config_env.rs:962` |
| 149 | supports github custom endpoint and github.com | ported | `crates/renovate-cli/src/config_env.rs:970` |
| 168 | supports github fine-grained pats | ported | `crates/renovate-cli/src/config_env.rs:987` |
| 185 | supports renovate_ prefixed github com token | ported | `crates/renovate-cli/src/config_env.rs:999` |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | `crates/renovate-cli/src/config_env.rs:1010` |
| 220 | supports github custom endpoint and gitlab.com | ported | `crates/renovate-cli/src/config_env.rs:1022` |
| 231 | supports gitlab token | ported | `crates/renovate-cli/src/config_env.rs:1035` |
| 242 | supports gitlab custom endpoint | ported | `crates/renovate-cli/src/config_env.rs:1047` |
| 255 | supports azure devops | ported | `crates/renovate-cli/src/config_env.rs:1061` |
| 268 | supports bitbucket token | ported | `crates/renovate-cli/src/config_env.rs:1075` |
| 283 | supports bitbucket username/password | ported | `crates/renovate-cli/src/config_env.rs:1091` |
| 299 | merges full config from env | ported | `crates/renovate-cli/src/config_env.rs:1108` |
| 309 | massages converted experimental env vars | ported | `crates/renovate-cli/src/config_env.rs:1132` |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | `crates/renovate-cli/src/config_env.rs:1225` |
| 357 | crashes | ported | `crates/renovate-cli/src/config_env.rs:1232` |
| 367 | migrates renovate_config | ported | `crates/renovate-cli/src/config_env.rs:1239` |
| 376 | warns if config in renovate_config is invalid | pending | — |
| 386 | renames migrated variables | ported | `crates/renovate-cli/src/config_env.rs:1251` |
| 396 | has no duplicate env names across options | ported | `crates/renovate-cli/src/config_env.rs:1687` |
| 418 | returns empty | ported | `crates/renovate-core/src/util.rs:5860` |
| 426 | returns existing env | ported | `crates/renovate-core/src/util.rs:5866` |
| 434 | generates renovate_ env | ported | `crates/renovate-core/src/util.rs:5872` |
| 441 | dryrun boolean true | ported | `crates/renovate-cli/src/config_env.rs:1580` |
| 449 | dryrun boolean false | ported | `crates/renovate-cli/src/config_env.rs:1587` |
| 457 | dryrun null | ported | `crates/renovate-cli/src/config_env.rs:1594` |
| 465 | requireconfig boolean true | ported | `crates/renovate-cli/src/config_env.rs:1601` |
| 473 | requireconfig boolean false | ported | `crates/renovate-cli/src/config_env.rs:1608` |
| 481 | platformcommit boolean true | ported | `crates/renovate-cli/src/config_env.rs:1660` |
| 489 | platformcommit boolean false | ported | `crates/renovate-cli/src/config_env.rs:1667` |


# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**44/45 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:684`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L684) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:692`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L692) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:699`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L699) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:783`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L783) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:793`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L793) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:800`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L800) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:807`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L807) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:829`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L829) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:836`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L836) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:889`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L889) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:897`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L897) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:904`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L904) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:930`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L930) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:948`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L948) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:955`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L955) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:963`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L963) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:971`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L971) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:988`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L988) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1000`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1000) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1011`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1011) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1023`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1023) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1036`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1036) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1048`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1048) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1062`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1062) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1076`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1076) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1092`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1092) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1109`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1109) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1133`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1133) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1226`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1226) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1233`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1233) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1240`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1240) |
| 376 | warns if config in renovate_config is invalid | pending | — |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1252`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1252) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1688`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1688) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6824`](../../../../../../../../crates/renovate-core/src/util.rs#L6824) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6830`](../../../../../../../../crates/renovate-core/src/util.rs#L6830) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6836`](../../../../../../../../crates/renovate-core/src/util.rs#L6836) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1581`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1581) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1588`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1588) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1595`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1595) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1602`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1602) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1609`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1609) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1661`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1661) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1668`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1668) |


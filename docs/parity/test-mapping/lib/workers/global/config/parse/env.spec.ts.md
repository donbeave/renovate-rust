# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**45/45 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:740`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L740) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:748`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L748) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:755`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L755) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:839`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L839) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:849`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L849) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:856`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L856) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:863`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L863) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:885`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L885) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:892`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L892) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:945`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L945) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:953`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L953) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:960`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L960) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:986`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L986) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:1004`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1004) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:1011`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1011) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1019`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1019) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:1027`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1027) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:1044`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1044) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1056`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1056) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1067`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1067) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1079`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1079) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1092`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1092) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1104`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1104) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1118`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1118) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1132`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1132) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1148`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1148) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1165`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1165) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1189`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1189) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1282`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1282) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1289`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1289) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1308`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1308) |
| 376 | warns if config in renovate_config is invalid | ported | [`crates/renovate-cli/src/config_env.rs:1296`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1296) |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1320`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1320) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1756`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1756) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6921`](../../../../../../../../crates/renovate-core/src/util.rs#L6921) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6927`](../../../../../../../../crates/renovate-core/src/util.rs#L6927) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6933`](../../../../../../../../crates/renovate-core/src/util.rs#L6933) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1649`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1649) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1656`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1656) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1663`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1663) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1670`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1670) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1677`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1677) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1729`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1729) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1736`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1736) |


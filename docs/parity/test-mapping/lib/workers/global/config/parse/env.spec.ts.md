# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**45/45 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:812`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L812) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:820`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L820) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:827`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L827) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:911`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L911) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:921`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L921) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:928`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L928) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:938`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L938) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:969`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L969) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:1015`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1015) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:1068`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1068) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:1076`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1076) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:1083`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1083) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:1109`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1109) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:1127`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1127) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:1134`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1134) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1142`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1142) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:1150`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1150) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:1167`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1167) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1179`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1179) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1204`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1204) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1234`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1234) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1247`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1247) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1259`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1259) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1273`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1273) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1287`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1287) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1304`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1304) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1321`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1321) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1345`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1345) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1441`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1441) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1448`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1448) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1455`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1455) |
| 376 | warns if config in renovate_config is invalid | ported | [`crates/renovate-cli/src/config_env.rs:1467`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1467) |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1479`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1479) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1963`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1963) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6825`](../../../../../../../../crates/renovate-core/src/util.rs#L6825) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6831`](../../../../../../../../crates/renovate-core/src/util.rs#L6831) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6837`](../../../../../../../../crates/renovate-core/src/util.rs#L6837) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1847`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1847) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1854`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1854) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1870`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1870) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1877`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1877) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1884`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1884) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1936`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1936) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1943`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1943) |


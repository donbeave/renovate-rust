# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**45/45 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:702`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L702) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:710`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L710) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:717`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L717) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:801`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L801) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:811`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L811) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:818`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L818) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:825`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L825) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:847`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L847) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:854`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L854) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:907`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L907) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:915`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L915) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:922`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L922) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:948`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L948) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:966`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L966) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:973`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L973) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:981`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L981) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:989`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L989) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:1006`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1006) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1018`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1018) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1029`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1029) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1041`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1041) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1054`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1054) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1066`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1066) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1080`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1080) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1094`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1094) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1110`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1110) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1127`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1127) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1151`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1151) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1244`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1244) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1251`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1251) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1258`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1258) |
| 376 | warns if config in renovate_config is invalid | ported | [`crates/renovate-cli/src/config_env.rs:1270`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1270) |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1282`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1282) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1718`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1718) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6825`](../../../../../../../../crates/renovate-core/src/util.rs#L6825) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6831`](../../../../../../../../crates/renovate-core/src/util.rs#L6831) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6837`](../../../../../../../../crates/renovate-core/src/util.rs#L6837) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1611`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1611) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1618`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1618) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1625`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1625) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1632`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1632) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1639`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1639) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1691`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1691) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1698`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1698) |


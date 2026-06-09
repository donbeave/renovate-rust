# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**44/45 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:696`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L696) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:704`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L704) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:711`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L711) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:795`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L795) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:805`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L805) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:812`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L812) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:819`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L819) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:841`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L841) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:848`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L848) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:901`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L901) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:909`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L909) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:916`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L916) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:942`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L942) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:960`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L960) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:967`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L967) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:975`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L975) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:983`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L983) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:1000`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1000) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1012`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1012) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1023`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1023) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1035`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1035) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1048`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1048) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1060`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1060) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1074`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1074) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1088`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1088) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1104`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1104) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1121`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1121) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1145`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1145) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1238`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1238) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1245`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1245) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1252`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1252) |
| 376 | warns if config in renovate_config is invalid | pending | — |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1264`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1264) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1700`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1700) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6824`](../../../../../../../../crates/renovate-core/src/util.rs#L6824) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6830`](../../../../../../../../crates/renovate-core/src/util.rs#L6830) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6836`](../../../../../../../../crates/renovate-core/src/util.rs#L6836) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1593`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1593) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1600`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1600) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1607`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1607) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1614`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1614) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1621`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1621) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1673`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1673) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1680`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1680) |


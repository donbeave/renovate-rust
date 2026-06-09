# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**45/45 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:703`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L703) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:711`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L711) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:718`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L718) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:802`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L802) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:812`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L812) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:819`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L819) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:826`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L826) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:848`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L848) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:873`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L873) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:926`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L926) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:934`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L934) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:941`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L941) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:967`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L967) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:985`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L985) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:992`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L992) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1000`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1000) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:1008`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1008) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:1025`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1025) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1037`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1037) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1048`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1048) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1060`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1060) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1073`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1073) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1085`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1085) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1099`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1099) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1113`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1113) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1130`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1130) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1147`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1147) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1171`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1171) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1264`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1264) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1271`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1271) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1278`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1278) |
| 376 | warns if config in renovate_config is invalid | ported | [`crates/renovate-cli/src/config_env.rs:1290`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1290) |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1302`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1302) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1739`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1739) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6825`](../../../../../../../../crates/renovate-core/src/util.rs#L6825) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6831`](../../../../../../../../crates/renovate-core/src/util.rs#L6831) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6837`](../../../../../../../../crates/renovate-core/src/util.rs#L6837) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1632`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1632) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1639`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1639) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1646`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1646) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1653`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1653) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1660`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1660) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1712`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1712) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1719`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1719) |


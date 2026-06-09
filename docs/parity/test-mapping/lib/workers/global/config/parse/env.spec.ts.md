# `lib/workers/global/config/parse/env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**44/45 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns empty env | ported | [`crates/renovate-cli/src/config_env.rs:683`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L683) |
| 15 | supports boolean true | ported | [`crates/renovate-cli/src/config_env.rs:691`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L691) |
| 20 | supports boolean false | ported | [`crates/renovate-cli/src/config_env.rs:698`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L698) |
| 27 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_env.rs:782`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L782) |
| 40 | supports list single | ported | [`crates/renovate-cli/src/config_env.rs:792`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L792) |
| 45 | supports list multiple | ported | [`crates/renovate-cli/src/config_env.rs:799`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L799) |
| 50 | supports list multiple without blank items | ported | [`crates/renovate-cli/src/config_env.rs:806`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L806) |
| 55 | supports string | ported | [`crates/renovate-cli/src/config_env.rs:828`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L828) |
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:835`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L835) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:888`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L888) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:896`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L896) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:903`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L903) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:929`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L929) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:947`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L947) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:954`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L954) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:962`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L962) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:970`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L970) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:987`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L987) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:999`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L999) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1010`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1010) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1022`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1022) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1035`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1035) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1047`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1047) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1061`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1061) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1075`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1075) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1091`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1091) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1108`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1108) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1132`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1132) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1225`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1225) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1232`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1232) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1239`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1239) |
| 376 | warns if config in renovate_config is invalid | pending | — |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1251`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1251) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1687`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1687) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6824`](../../../../../../../../crates/renovate-core/src/util.rs#L6824) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6830`](../../../../../../../../crates/renovate-core/src/util.rs#L6830) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6836`](../../../../../../../../crates/renovate-core/src/util.rs#L6836) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1580`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1580) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1587`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1587) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1594`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1594) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1601`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1601) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1608`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1608) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1660`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1660) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1667`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1667) |


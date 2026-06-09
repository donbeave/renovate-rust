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
| 60 | coerces string newlines | ported | [`crates/renovate-cli/src/config_env.rs:872`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L872) |
| 67 | supports custom prefixes | ported | [`crates/renovate-cli/src/config_env.rs:925`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L925) |
| 76 | supports json | ported | [`crates/renovate-cli/src/config_env.rs:933`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L933) |
| 83 | supports arrays of objects | ported | [`crates/renovate-cli/src/config_env.rs:940`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L940) |
| 91 | _(it.each / template — verify manually)_ | ? | — |
| 103 | skips misconfigured arrays | ported | [`crates/renovate-cli/src/config_env.rs:966`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L966) |
| 117 | skips garbage array values | ported | [`crates/renovate-cli/src/config_env.rs:984`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L984) |
| 131 | supports github token | ported | [`crates/renovate-cli/src/config_env.rs:991`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L991) |
| 140 | supports github custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:999`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L999) |
| 149 | supports github custom endpoint and github.com | ported | [`crates/renovate-cli/src/config_env.rs:1007`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1007) |
| 168 | supports github fine-grained pats | ported | [`crates/renovate-cli/src/config_env.rs:1024`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1024) |
| 185 | supports renovate_ prefixed github com token | ported | [`crates/renovate-cli/src/config_env.rs:1036`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1036) |
| 202 | github_com_token takes precedence over renovate_github_com_token | ported | [`crates/renovate-cli/src/config_env.rs:1047`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1047) |
| 220 | supports github custom endpoint and gitlab.com | ported | [`crates/renovate-cli/src/config_env.rs:1059`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1059) |
| 231 | supports gitlab token | ported | [`crates/renovate-cli/src/config_env.rs:1072`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1072) |
| 242 | supports gitlab custom endpoint | ported | [`crates/renovate-cli/src/config_env.rs:1084`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1084) |
| 255 | supports azure devops | ported | [`crates/renovate-cli/src/config_env.rs:1098`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1098) |
| 268 | supports bitbucket token | ported | [`crates/renovate-cli/src/config_env.rs:1112`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1112) |
| 283 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1129`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1129) |
| 299 | merges full config from env | ported | [`crates/renovate-cli/src/config_env.rs:1146`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1146) |
| 309 | massages converted experimental env vars | ported | [`crates/renovate-cli/src/config_env.rs:1170`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1170) |
| 336 | does not migrate empty renovate_x_repo_cache_force_local | ported | [`crates/renovate-cli/src/config_env.rs:1263`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1263) |
| 357 | crashes | ported | [`crates/renovate-cli/src/config_env.rs:1270`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1270) |
| 367 | migrates renovate_config | ported | [`crates/renovate-cli/src/config_env.rs:1277`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1277) |
| 376 | warns if config in renovate_config is invalid | ported | [`crates/renovate-cli/src/config_env.rs:1289`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1289) |
| 386 | renames migrated variables | ported | [`crates/renovate-cli/src/config_env.rs:1301`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1301) |
| 396 | has no duplicate env names across options | ported | [`crates/renovate-cli/src/config_env.rs:1738`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1738) |
| 418 | returns empty | ported | [`crates/renovate-core/src/util.rs:6825`](../../../../../../../../crates/renovate-core/src/util.rs#L6825) |
| 426 | returns existing env | ported | [`crates/renovate-core/src/util.rs:6831`](../../../../../../../../crates/renovate-core/src/util.rs#L6831) |
| 434 | generates renovate_ env | ported | [`crates/renovate-core/src/util.rs:6837`](../../../../../../../../crates/renovate-core/src/util.rs#L6837) |
| 441 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1631`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1631) |
| 449 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1638`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1638) |
| 457 | dryrun null | ported | [`crates/renovate-cli/src/config_env.rs:1645`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1645) |
| 465 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1652`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1652) |
| 473 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1659`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1659) |
| 481 | platformcommit boolean true | ported | [`crates/renovate-cli/src/config_env.rs:1711`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1711) |
| 489 | platformcommit boolean false | ported | [`crates/renovate-cli/src/config_env.rs:1718`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1718) |


# `lib/workers/global/config/parse/index.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**15/35 in-scope tests ported** (20 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 44 | supports token in env | ported | [`crates/renovate-cli/src/config_env.rs:849`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L849) |
| 51 | supports token in cli options | ported | [`crates/renovate-cli/src/config_builder.rs:668`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L668) |
| 69 | supports forcecli | pending | — |
| 84 | sets customenvvariables | ported | [`crates/renovate-cli/src/config_env.rs:855`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L855) |
| 98 | supports config.force | ported | [`crates/renovate-core/src/config.rs:857`](../../../../../../../../crates/renovate-core/src/config.rs#L857) |
| 120 | reads private key from file | pending | — |
| 145 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1128`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1128) |
| 163 | massages trailing slash into endpoint | ported | [`crates/renovate-cli/src/config_builder.rs:1241`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1241) |
| 172 | parses global manager config | ported | [`crates/renovate-cli/src/config_builder.rs:865`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L865) |
| 179 | parses host rules from env | ported | [`crates/renovate-cli/src/config_builder.rs:1255`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1255) |
| 187 | env dryrun = true replaced to full | ported | [`crates/renovate-cli/src/config_env.rs:1630`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1630) |
| 197 | cli dryrun = true replaced to full | ported | [`crates/renovate-cli/src/config_builder.rs:1208`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1208) |
| 204 | resolves global presets | ported | [`crates/renovate-cli/src/config_builder.rs:866`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L866) |
| 232 | throws exception if global presets cannot be resolved | pending | — |
| 247 | cli dryrun replaced to full | ported | [`crates/renovate-cli/src/config_builder.rs:1219`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1219) |
| 254 | env dryrun = false replaced to null | ported | [`crates/renovate-cli/src/config_env.rs:1645`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1645) |
| 264 | cli dryrun = false replaced to null | ported | [`crates/renovate-cli/src/config_builder.rs:1232`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1232) |
| 271 | only initializes the file when the env var log_file is properly set | pending | — |
| 278 | massage onboardingnodeps when autodiscover is false | pending | — |
| 289 | does not massage onboardingnodeps when autodiscover is true | pending | — |
| 299 | apply secrets to global config | pending | — |
| 319 | overrides file config with additional file config | pending | — |
| 334 | merges extends from file config with additional file config | pending | — |
| 352 | adds extends from fileconfig only | pending | — |
| 363 | appends files from configfilenames to config filenames list | pending | — |
| 380 | supports setting configfilenames through cli | ported | [`crates/renovate-cli/src/config_builder.rs:1268`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1268) |
| 391 | supports setting configfilenames through env | pending | — |
| 405 | warns when cli config overrides repositories from file config | pending | — |
| 416 | warns when cli config overrides repositories from env config | pending | — |
| 429 | does not warn when cli config sets repositories without override | pending | — |
| 438 | does not warn when cli config has no repositories | pending | — |
| 448 | does not warn when cli config has same repositories as file config | pending | — |
| 459 | warns when cli overrides repositories with repo-specific configuration | pending | — |
| 475 | does not warn when both values are the same | pending | — |
| 487 | warns when both values are effectively the same | pending | — |


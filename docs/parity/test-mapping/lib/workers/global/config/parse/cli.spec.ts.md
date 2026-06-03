# `lib/workers/global/config/parse/cli.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**30/30 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 15 | generates cli value | ported | `crates/renovate-core/src/util.rs:5885` |
| 22 | generates returns empty if cli false | ported | `crates/renovate-core/src/util.rs:5891` |
| 32 | returns empty argv | ported | `crates/renovate-cli/src/config_builder.rs:631` |
| 36 | supports boolean no value | ported | `crates/renovate-cli/src/config_builder.rs:919` |
| 42 | supports boolean space true | ported | `crates/renovate-cli/src/config_builder.rs:991` |
| 48 | throws exception for invalid boolean value | ported | `crates/renovate-cli/src/config_builder.rs:997` |
| 58 | supports boolean space false | ported | `crates/renovate-cli/src/config_builder.rs:1005` |
| 64 | supports boolean equals true | ported | `crates/renovate-cli/src/config_builder.rs:1011` |
| 69 | supports boolean equals false | ported | `crates/renovate-cli/src/config_builder.rs:1017` |
| 74 | supports list single | ported | `crates/renovate-cli/src/config_builder.rs:1072` |
| 79 | supports list multiple | ported | `crates/renovate-cli/src/config_builder.rs:1078` |
| 84 | supports string | ported | `crates/renovate-cli/src/config_builder.rs:661` |
| 89 | supports repositories | ported | `crates/renovate-cli/src/config_builder.rs:731` |
| 95 | parses json lists correctly | ported | `crates/renovate-cli/src/config_builder.rs:1087` |
| 111 | parses [] correctly as empty list of hostrules | ported | `crates/renovate-cli/src/config_builder.rs:1100` |
| 118 | parses an empty string correctly as empty list of hostrules | ported | `crates/renovate-cli/src/config_builder.rs:1106` |
| 125 | _(it.each / template — verify manually)_ | ? | — |
| 145 | parses json object correctly when empty | ported | `crates/renovate-cli/src/config_builder.rs:1144` |
| 152 | parses json {} object correctly | ported | `crates/renovate-cli/src/config_builder.rs:1154` |
| 159 | parses json object correctly | ported | `crates/renovate-cli/src/config_builder.rs:1164` |
| 168 | throws exception for invalid json object | ported | `crates/renovate-cli/src/config_builder.rs:1179` |
| 175 | dryrun boolean true | ported | `crates/renovate-cli/src/config_builder.rs:1198` |
| 180 | dryrun no value | ported | `crates/renovate-cli/tests/cli.rs:134` |
| 185 | dryrun boolean false | ported | `crates/renovate-cli/src/config_builder.rs:1208` |
| 190 | dryrun null | ported | `crates/renovate-cli/src/config_builder.rs:1215` |
| 195 | requireconfig boolean true | ported | `crates/renovate-cli/src/config_builder.rs:1222` |
| 200 | requireconfig no value | ported | `crates/renovate-cli/tests/cli.rs:160` |
| 205 | requireconfig boolean false | ported | `crates/renovate-cli/src/config_builder.rs:1232` |
| 212 | prints version and exits when --version is passed | ported | `crates/renovate-cli/tests/cli.rs:17` |
| 229 | does not error when --dry-run is the last argument | ported | `crates/renovate-cli/tests/cli.rs:142` |


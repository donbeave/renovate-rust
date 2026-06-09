# `lib/workers/global/config/parse/cli.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | generates cli value | ported | [`crates/renovate-core/src/util.rs:6946`](../../../../../../../../crates/renovate-core/src/util.rs#L6946) |
| 22 | generates returns empty if cli false | ported | [`crates/renovate-core/src/util.rs:6952`](../../../../../../../../crates/renovate-core/src/util.rs#L6952) |
| 32 | returns empty argv | ported | [`crates/renovate-cli/src/config_builder.rs:637`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L637) |
| 36 | supports boolean no value | ported | [`crates/renovate-cli/src/config_builder.rs:925`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L925) |
| 42 | supports boolean space true | ported | [`crates/renovate-cli/src/config_builder.rs:997`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L997) |
| 48 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_builder.rs:1003`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1003) |
| 58 | supports boolean space false | ported | [`crates/renovate-cli/src/config_builder.rs:1011`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1011) |
| 64 | supports boolean equals true | ported | [`crates/renovate-cli/src/config_builder.rs:1017`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1017) |
| 69 | supports boolean equals false | ported | [`crates/renovate-cli/src/config_builder.rs:1023`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1023) |
| 74 | supports list single | ported | [`crates/renovate-cli/src/config_builder.rs:1078`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1078) |
| 79 | supports list multiple | ported | [`crates/renovate-cli/src/config_builder.rs:1084`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1084) |
| 84 | supports string | ported | [`crates/renovate-cli/src/config_builder.rs:667`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L667) |
| 89 | supports repositories | ported | [`crates/renovate-cli/src/config_builder.rs:737`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L737) |
| 95 | parses json lists correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1093`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1093) |
| 111 | parses [] correctly as empty list of hostrules | ported | [`crates/renovate-cli/src/config_builder.rs:1106`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1106) |
| 118 | parses an empty string correctly as empty list of hostrules | ported | [`crates/renovate-cli/src/config_builder.rs:1112`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1112) |
| 125 | _(it.each / template — verify manually)_ | ? | — |
| 145 | parses json object correctly when empty | ported | [`crates/renovate-cli/src/config_builder.rs:1150`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1150) |
| 152 | parses json {} object correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1160`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1160) |
| 159 | parses json object correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1170`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1170) |
| 168 | throws exception for invalid json object | ported | [`crates/renovate-cli/src/config_builder.rs:1185`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1185) |
| 175 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_builder.rs:1204`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1204) |
| 180 | dryrun no value | ported | [`crates/renovate-cli/tests/cli.rs:134`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L134) |
| 185 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_builder.rs:1214`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1214) |
| 190 | dryrun null | ported | [`crates/renovate-cli/src/config_builder.rs:1221`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1221) |
| 195 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_builder.rs:1228`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1228) |
| 200 | requireconfig no value | ported | [`crates/renovate-cli/tests/cli.rs:160`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L160) |
| 205 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_builder.rs:1238`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1238) |
| 212 | prints version and exits when --version is passed | ported | [`crates/renovate-cli/tests/cli.rs:17`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L17) |
| 229 | does not error when --dry-run is the last argument | ported | [`crates/renovate-cli/tests/cli.rs:142`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L142) |


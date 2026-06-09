# `lib/workers/global/config/parse/cli.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | generates cli value | ported | [`crates/renovate-core/src/util.rs:6850`](../../../../../../../../crates/renovate-core/src/util.rs#L6850) |
| 22 | generates returns empty if cli false | ported | [`crates/renovate-core/src/util.rs:6856`](../../../../../../../../crates/renovate-core/src/util.rs#L6856) |
| 32 | returns empty argv | ported | [`crates/renovate-cli/src/config_builder.rs:648`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L648) |
| 36 | supports boolean no value | ported | [`crates/renovate-cli/src/config_builder.rs:939`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L939) |
| 42 | supports boolean space true | ported | [`crates/renovate-cli/src/config_builder.rs:1011`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1011) |
| 48 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_builder.rs:1017`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1017) |
| 58 | supports boolean space false | ported | [`crates/renovate-cli/src/config_builder.rs:1025`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1025) |
| 64 | supports boolean equals true | ported | [`crates/renovate-cli/src/config_builder.rs:1031`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1031) |
| 69 | supports boolean equals false | ported | [`crates/renovate-cli/src/config_builder.rs:1037`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1037) |
| 74 | supports list single | ported | [`crates/renovate-cli/src/config_builder.rs:1092`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1092) |
| 79 | supports list multiple | ported | [`crates/renovate-cli/src/config_builder.rs:1098`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1098) |
| 84 | supports string | ported | [`crates/renovate-cli/src/config_builder.rs:679`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L679) |
| 89 | supports repositories | ported | [`crates/renovate-cli/src/config_builder.rs:749`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L749) |
| 95 | parses json lists correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1107`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1107) |
| 111 | parses [] correctly as empty list of hostrules | ported | [`crates/renovate-cli/src/config_builder.rs:1120`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1120) |
| 118 | parses an empty string correctly as empty list of hostrules | ported | [`crates/renovate-cli/src/config_builder.rs:1126`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1126) |
| 125 | _(it.each / template — verify manually)_ | ? | — |
| 145 | parses json object correctly when empty | ported | [`crates/renovate-cli/src/config_builder.rs:1164`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1164) |
| 152 | parses json {} object correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1174`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1174) |
| 159 | parses json object correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1184`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1184) |
| 168 | throws exception for invalid json object | ported | [`crates/renovate-cli/src/config_builder.rs:1199`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1199) |
| 175 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_builder.rs:1219`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1219) |
| 180 | dryrun no value | ported | [`crates/renovate-cli/tests/cli.rs:134`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L134) |
| 185 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_builder.rs:1313`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1313) |
| 190 | dryrun null | ported | [`crates/renovate-cli/src/config_builder.rs:1320`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1320) |
| 195 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_builder.rs:1327`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1327) |
| 200 | requireconfig no value | ported | [`crates/renovate-cli/tests/cli.rs:160`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L160) |
| 205 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_builder.rs:1337`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1337) |
| 212 | prints version and exits when --version is passed | ported | [`crates/renovate-cli/tests/cli.rs:17`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L17) |
| 229 | does not error when --dry-run is the last argument | ported | [`crates/renovate-cli/tests/cli.rs:142`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L142) |


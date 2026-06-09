# `lib/workers/global/config/parse/cli.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | generates cli value | ported | [`crates/renovate-core/src/util.rs:6853`](../../../../../../../../crates/renovate-core/src/util.rs#L6853) |
| 22 | generates returns empty if cli false | ported | [`crates/renovate-core/src/util.rs:6859`](../../../../../../../../crates/renovate-core/src/util.rs#L6859) |
| 32 | returns empty argv | ported | [`crates/renovate-cli/src/config_builder.rs:631`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L631) |
| 36 | supports boolean no value | ported | [`crates/renovate-cli/src/config_builder.rs:922`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L922) |
| 42 | supports boolean space true | ported | [`crates/renovate-cli/src/config_builder.rs:994`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L994) |
| 48 | throws exception for invalid boolean value | ported | [`crates/renovate-cli/src/config_builder.rs:1000`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1000) |
| 58 | supports boolean space false | ported | [`crates/renovate-cli/src/config_builder.rs:1008`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1008) |
| 64 | supports boolean equals true | ported | [`crates/renovate-cli/src/config_builder.rs:1014`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1014) |
| 69 | supports boolean equals false | ported | [`crates/renovate-cli/src/config_builder.rs:1020`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1020) |
| 74 | supports list single | ported | [`crates/renovate-cli/src/config_builder.rs:1075`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1075) |
| 79 | supports list multiple | ported | [`crates/renovate-cli/src/config_builder.rs:1081`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1081) |
| 84 | supports string | ported | [`crates/renovate-cli/src/config_builder.rs:662`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L662) |
| 89 | supports repositories | ported | [`crates/renovate-cli/src/config_builder.rs:732`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L732) |
| 95 | parses json lists correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1090`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1090) |
| 111 | parses [] correctly as empty list of hostrules | ported | [`crates/renovate-cli/src/config_builder.rs:1103`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1103) |
| 118 | parses an empty string correctly as empty list of hostrules | ported | [`crates/renovate-cli/src/config_builder.rs:1109`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1109) |
| 125 | _(it.each / template — verify manually)_ | ? | — |
| 145 | parses json object correctly when empty | ported | [`crates/renovate-cli/src/config_builder.rs:1147`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1147) |
| 152 | parses json {} object correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1157`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1157) |
| 159 | parses json object correctly | ported | [`crates/renovate-cli/src/config_builder.rs:1167`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1167) |
| 168 | throws exception for invalid json object | ported | [`crates/renovate-cli/src/config_builder.rs:1182`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1182) |
| 175 | dryrun boolean true | ported | [`crates/renovate-cli/src/config_builder.rs:1202`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1202) |
| 180 | dryrun no value | ported | [`crates/renovate-cli/tests/cli.rs:134`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L134) |
| 185 | dryrun boolean false | ported | [`crates/renovate-cli/src/config_builder.rs:1212`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1212) |
| 190 | dryrun null | ported | [`crates/renovate-cli/src/config_builder.rs:1219`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1219) |
| 195 | requireconfig boolean true | ported | [`crates/renovate-cli/src/config_builder.rs:1226`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1226) |
| 200 | requireconfig no value | ported | [`crates/renovate-cli/tests/cli.rs:160`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L160) |
| 205 | requireconfig boolean false | ported | [`crates/renovate-cli/src/config_builder.rs:1236`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1236) |
| 212 | prints version and exits when --version is passed | ported | [`crates/renovate-cli/tests/cli.rs:17`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L17) |
| 229 | does not error when --dry-run is the last argument | ported | [`crates/renovate-cli/tests/cli.rs:142`](../../../../../../../../crates/renovate-cli/tests/cli.rs#L142) |


# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**13/15 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12046`](../../../../../crates/renovate-core/src/util.rs#L12046) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12052`](../../../../../crates/renovate-core/src/util.rs#L12052) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:12059`](../../../../../crates/renovate-core/src/util.rs#L12059) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:12066`](../../../../../crates/renovate-core/src/util.rs#L12066) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:12080`](../../../../../crates/renovate-core/src/util.rs#L12080) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:12095`](../../../../../crates/renovate-core/src/util.rs#L12095) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12046`](../../../../../crates/renovate-core/src/util.rs#L12046) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12052`](../../../../../crates/renovate-core/src/util.rs#L12052) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:12120`](../../../../../crates/renovate-core/src/util.rs#L12120) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:12127`](../../../../../crates/renovate-core/src/util.rs#L12127) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:12138`](../../../../../crates/renovate-core/src/util.rs#L12138) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:12155`](../../../../../crates/renovate-core/src/util.rs#L12155) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:12171`](../../../../../crates/renovate-core/src/util.rs#L12171) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:12185`](../../../../../crates/renovate-core/src/util.rs#L12185) |
| 175 | writes formatted data to stdout | ported | [`crates/renovate-core/src/util.rs:12226`](../../../../../crates/renovate-core/src/util.rs#L12226) |


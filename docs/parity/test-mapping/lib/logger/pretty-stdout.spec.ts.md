# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**13/15 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12044`](../../../../../crates/renovate-core/src/util.rs#L12044) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12050`](../../../../../crates/renovate-core/src/util.rs#L12050) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:12057`](../../../../../crates/renovate-core/src/util.rs#L12057) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:12064`](../../../../../crates/renovate-core/src/util.rs#L12064) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:12078`](../../../../../crates/renovate-core/src/util.rs#L12078) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:12093`](../../../../../crates/renovate-core/src/util.rs#L12093) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12044`](../../../../../crates/renovate-core/src/util.rs#L12044) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12050`](../../../../../crates/renovate-core/src/util.rs#L12050) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:12118`](../../../../../crates/renovate-core/src/util.rs#L12118) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:12125`](../../../../../crates/renovate-core/src/util.rs#L12125) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:12136`](../../../../../crates/renovate-core/src/util.rs#L12136) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:12153`](../../../../../crates/renovate-core/src/util.rs#L12153) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:12169`](../../../../../crates/renovate-core/src/util.rs#L12169) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:12183`](../../../../../crates/renovate-core/src/util.rs#L12183) |
| 175 | writes formatted data to stdout | ported | [`crates/renovate-core/src/util.rs:12224`](../../../../../crates/renovate-core/src/util.rs#L12224) |


# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**13/15 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12049`](../../../../../crates/renovate-core/src/util.rs#L12049) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12055`](../../../../../crates/renovate-core/src/util.rs#L12055) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:12062`](../../../../../crates/renovate-core/src/util.rs#L12062) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:12069`](../../../../../crates/renovate-core/src/util.rs#L12069) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:12083`](../../../../../crates/renovate-core/src/util.rs#L12083) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:12098`](../../../../../crates/renovate-core/src/util.rs#L12098) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12049`](../../../../../crates/renovate-core/src/util.rs#L12049) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12055`](../../../../../crates/renovate-core/src/util.rs#L12055) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:12123`](../../../../../crates/renovate-core/src/util.rs#L12123) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:12130`](../../../../../crates/renovate-core/src/util.rs#L12130) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:12141`](../../../../../crates/renovate-core/src/util.rs#L12141) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:12158`](../../../../../crates/renovate-core/src/util.rs#L12158) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:12174`](../../../../../crates/renovate-core/src/util.rs#L12174) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:12188`](../../../../../crates/renovate-core/src/util.rs#L12188) |
| 175 | writes formatted data to stdout | ported | [`crates/renovate-core/src/util.rs:12229`](../../../../../crates/renovate-core/src/util.rs#L12229) |


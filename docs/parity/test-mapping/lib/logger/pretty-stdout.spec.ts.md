# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**13/15 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12141`](../../../../../crates/renovate-core/src/util.rs#L12141) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12147`](../../../../../crates/renovate-core/src/util.rs#L12147) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:12154`](../../../../../crates/renovate-core/src/util.rs#L12154) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:12161`](../../../../../crates/renovate-core/src/util.rs#L12161) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:12175`](../../../../../crates/renovate-core/src/util.rs#L12175) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:12190`](../../../../../crates/renovate-core/src/util.rs#L12190) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12141`](../../../../../crates/renovate-core/src/util.rs#L12141) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12147`](../../../../../crates/renovate-core/src/util.rs#L12147) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:12215`](../../../../../crates/renovate-core/src/util.rs#L12215) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:12222`](../../../../../crates/renovate-core/src/util.rs#L12222) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:12233`](../../../../../crates/renovate-core/src/util.rs#L12233) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:12250`](../../../../../crates/renovate-core/src/util.rs#L12250) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:12266`](../../../../../crates/renovate-core/src/util.rs#L12266) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:12280`](../../../../../crates/renovate-core/src/util.rs#L12280) |
| 175 | writes formatted data to stdout | ported | [`crates/renovate-core/src/util.rs:12321`](../../../../../crates/renovate-core/src/util.rs#L12321) |


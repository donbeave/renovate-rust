# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**13/15 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12045`](../../../../../crates/renovate-core/src/util.rs#L12045) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12051`](../../../../../crates/renovate-core/src/util.rs#L12051) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:12058`](../../../../../crates/renovate-core/src/util.rs#L12058) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:12065`](../../../../../crates/renovate-core/src/util.rs#L12065) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:12079`](../../../../../crates/renovate-core/src/util.rs#L12079) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:12094`](../../../../../crates/renovate-core/src/util.rs#L12094) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12045`](../../../../../crates/renovate-core/src/util.rs#L12045) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12051`](../../../../../crates/renovate-core/src/util.rs#L12051) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:12119`](../../../../../crates/renovate-core/src/util.rs#L12119) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:12126`](../../../../../crates/renovate-core/src/util.rs#L12126) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:12137`](../../../../../crates/renovate-core/src/util.rs#L12137) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:12154`](../../../../../crates/renovate-core/src/util.rs#L12154) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:12170`](../../../../../crates/renovate-core/src/util.rs#L12170) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:12184`](../../../../../crates/renovate-core/src/util.rs#L12184) |
| 175 | writes formatted data to stdout | ported | [`crates/renovate-core/src/util.rs:12225`](../../../../../crates/renovate-core/src/util.rs#L12225) |


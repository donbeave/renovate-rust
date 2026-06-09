# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**13/15 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12059`](../../../../../crates/renovate-core/src/util.rs#L12059) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12065`](../../../../../crates/renovate-core/src/util.rs#L12065) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:12072`](../../../../../crates/renovate-core/src/util.rs#L12072) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:12079`](../../../../../crates/renovate-core/src/util.rs#L12079) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:12093`](../../../../../crates/renovate-core/src/util.rs#L12093) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:12108`](../../../../../crates/renovate-core/src/util.rs#L12108) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12059`](../../../../../crates/renovate-core/src/util.rs#L12059) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12065`](../../../../../crates/renovate-core/src/util.rs#L12065) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:12133`](../../../../../crates/renovate-core/src/util.rs#L12133) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:12140`](../../../../../crates/renovate-core/src/util.rs#L12140) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:12151`](../../../../../crates/renovate-core/src/util.rs#L12151) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:12168`](../../../../../crates/renovate-core/src/util.rs#L12168) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:12184`](../../../../../crates/renovate-core/src/util.rs#L12184) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:12198`](../../../../../crates/renovate-core/src/util.rs#L12198) |
| 175 | writes formatted data to stdout | ported | [`crates/renovate-core/src/util.rs:12239`](../../../../../crates/renovate-core/src/util.rs#L12239) |


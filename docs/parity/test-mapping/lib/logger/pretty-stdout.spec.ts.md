# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**13/15 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12043`](../../../../../crates/renovate-core/src/util.rs#L12043) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12049`](../../../../../crates/renovate-core/src/util.rs#L12049) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:12056`](../../../../../crates/renovate-core/src/util.rs#L12056) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:12063`](../../../../../crates/renovate-core/src/util.rs#L12063) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:12077`](../../../../../crates/renovate-core/src/util.rs#L12077) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:12092`](../../../../../crates/renovate-core/src/util.rs#L12092) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:12043`](../../../../../crates/renovate-core/src/util.rs#L12043) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:12049`](../../../../../crates/renovate-core/src/util.rs#L12049) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:12117`](../../../../../crates/renovate-core/src/util.rs#L12117) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:12124`](../../../../../crates/renovate-core/src/util.rs#L12124) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:12135`](../../../../../crates/renovate-core/src/util.rs#L12135) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:12152`](../../../../../crates/renovate-core/src/util.rs#L12152) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:12168`](../../../../../crates/renovate-core/src/util.rs#L12168) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:12182`](../../../../../crates/renovate-core/src/util.rs#L12182) |
| 175 | writes formatted data to stdout | ported | [`crates/renovate-core/src/util.rs:12223`](../../../../../crates/renovate-core/src/util.rs#L12223) |


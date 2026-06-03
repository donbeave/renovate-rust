# `lib/logger/pretty-stdout.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**12/15 ported** (3 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:10362`](../../../../../crates/renovate-core/src/util.rs#L10362) |
| 13 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:10368`](../../../../../crates/renovate-core/src/util.rs#L10368) |
| 17 | returns empty string if no meta fields | ported | [`crates/renovate-core/src/util.rs:10375`](../../../../../crates/renovate-core/src/util.rs#L10375) |
| 24 | supports single meta | ported | [`crates/renovate-core/src/util.rs:10382`](../../../../../crates/renovate-core/src/util.rs#L10382) |
| 34 | supports multi meta | ported | [`crates/renovate-core/src/util.rs:10396`](../../../../../crates/renovate-core/src/util.rs#L10396) |
| 46 | returns plain text when colorize is false | ported | [`crates/renovate-core/src/util.rs:10411`](../../../../../crates/renovate-core/src/util.rs#L10411) |
| 57 | returns empty string if null rec | ported | [`crates/renovate-core/src/util.rs:10362`](../../../../../crates/renovate-core/src/util.rs#L10362) |
| 61 | returns empty string if empty rec | ported | [`crates/renovate-core/src/util.rs:10368`](../../../../../crates/renovate-core/src/util.rs#L10368) |
| 67 | returns empty string if all are meta fields | ported | [`crates/renovate-core/src/util.rs:10436`](../../../../../crates/renovate-core/src/util.rs#L10436) |
| 75 | supports a config | ported | [`crates/renovate-core/src/util.rs:10443`](../../../../../crates/renovate-core/src/util.rs#L10443) |
| 88 | formats err.stack as readable multi-line output | ported | [`crates/renovate-core/src/util.rs:10454`](../../../../../crates/renovate-core/src/util.rs#L10454) |
| 108 | formats err.stack without other err fields | ported | [`crates/renovate-core/src/util.rs:10471`](../../../../../crates/renovate-core/src/util.rs#L10471) |
| 136 | formats record | ported | [`crates/renovate-core/src/util.rs:10487`](../../../../../crates/renovate-core/src/util.rs#L10487) |
| 155 | formats record without colors | ported | [`crates/renovate-core/src/util.rs:10501`](../../../../../crates/renovate-core/src/util.rs#L10501) |
| 175 | writes formatted data to stdout | pending | — |


# `lib/logger/once.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should call a function only once | ported | [`crates/renovate-core/src/util.rs:8132`](../../../../../crates/renovate-core/src/util.rs#L8132) |
| 28 | supports support distinct calls | ported | [`crates/renovate-core/src/util.rs:8143`](../../../../../crates/renovate-core/src/util.rs#L8143) |
| 44 | resets keys | ported | [`crates/renovate-core/src/util.rs:8160`](../../../../../crates/renovate-core/src/util.rs#L8160) |
| 60 | logs once per function call | ported | [`crates/renovate-core/src/util.rs:8171`](../../../../../crates/renovate-core/src/util.rs#L8171) |
| 73 | distincts between log levels | ported | [`crates/renovate-core/src/util.rs:8188`](../../../../../crates/renovate-core/src/util.rs#L8188) |
| 89 | distincts between different log statements | ported | [`crates/renovate-core/src/util.rs:8208`](../../../../../crates/renovate-core/src/util.rs#L8208) |
| 106 | parameters are taken into account when de-duplicating calls | ported | [`crates/renovate-core/src/util.rs:8233`](../../../../../crates/renovate-core/src/util.rs#L8233) |
| 124 | allows mixing single-time and regular logging | ported | [`crates/renovate-core/src/util.rs:8279`](../../../../../crates/renovate-core/src/util.rs#L8279) |
| 146 | supports reset method | ported | [`crates/renovate-core/src/util.rs:8307`](../../../../../crates/renovate-core/src/util.rs#L8307) |


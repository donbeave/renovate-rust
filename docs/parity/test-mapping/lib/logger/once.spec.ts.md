# `lib/logger/once.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should call a function only once | ported | [`crates/renovate-core/src/util.rs:8137`](../../../../../crates/renovate-core/src/util.rs#L8137) |
| 28 | supports support distinct calls | ported | [`crates/renovate-core/src/util.rs:8148`](../../../../../crates/renovate-core/src/util.rs#L8148) |
| 44 | resets keys | ported | [`crates/renovate-core/src/util.rs:8165`](../../../../../crates/renovate-core/src/util.rs#L8165) |
| 60 | logs once per function call | ported | [`crates/renovate-core/src/util.rs:8176`](../../../../../crates/renovate-core/src/util.rs#L8176) |
| 73 | distincts between log levels | ported | [`crates/renovate-core/src/util.rs:8193`](../../../../../crates/renovate-core/src/util.rs#L8193) |
| 89 | distincts between different log statements | ported | [`crates/renovate-core/src/util.rs:8213`](../../../../../crates/renovate-core/src/util.rs#L8213) |
| 106 | parameters are taken into account when de-duplicating calls | ported | [`crates/renovate-core/src/util.rs:8238`](../../../../../crates/renovate-core/src/util.rs#L8238) |
| 124 | allows mixing single-time and regular logging | ported | [`crates/renovate-core/src/util.rs:8284`](../../../../../crates/renovate-core/src/util.rs#L8284) |
| 146 | supports reset method | ported | [`crates/renovate-core/src/util.rs:8312`](../../../../../crates/renovate-core/src/util.rs#L8312) |


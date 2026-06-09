# `lib/logger/once.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should call a function only once | ported | [`crates/renovate-core/src/util.rs:8131`](../../../../../crates/renovate-core/src/util.rs#L8131) |
| 28 | supports support distinct calls | ported | [`crates/renovate-core/src/util.rs:8142`](../../../../../crates/renovate-core/src/util.rs#L8142) |
| 44 | resets keys | ported | [`crates/renovate-core/src/util.rs:8159`](../../../../../crates/renovate-core/src/util.rs#L8159) |
| 60 | logs once per function call | ported | [`crates/renovate-core/src/util.rs:8170`](../../../../../crates/renovate-core/src/util.rs#L8170) |
| 73 | distincts between log levels | ported | [`crates/renovate-core/src/util.rs:8187`](../../../../../crates/renovate-core/src/util.rs#L8187) |
| 89 | distincts between different log statements | ported | [`crates/renovate-core/src/util.rs:8207`](../../../../../crates/renovate-core/src/util.rs#L8207) |
| 106 | parameters are taken into account when de-duplicating calls | ported | [`crates/renovate-core/src/util.rs:8232`](../../../../../crates/renovate-core/src/util.rs#L8232) |
| 124 | allows mixing single-time and regular logging | ported | [`crates/renovate-core/src/util.rs:8278`](../../../../../crates/renovate-core/src/util.rs#L8278) |
| 146 | supports reset method | ported | [`crates/renovate-core/src/util.rs:8306`](../../../../../crates/renovate-core/src/util.rs#L8306) |


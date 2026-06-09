# `lib/logger/once.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should call a function only once | ported | [`crates/renovate-core/src/util.rs:8134`](../../../../../crates/renovate-core/src/util.rs#L8134) |
| 28 | supports support distinct calls | ported | [`crates/renovate-core/src/util.rs:8145`](../../../../../crates/renovate-core/src/util.rs#L8145) |
| 44 | resets keys | ported | [`crates/renovate-core/src/util.rs:8162`](../../../../../crates/renovate-core/src/util.rs#L8162) |
| 60 | logs once per function call | ported | [`crates/renovate-core/src/util.rs:8173`](../../../../../crates/renovate-core/src/util.rs#L8173) |
| 73 | distincts between log levels | ported | [`crates/renovate-core/src/util.rs:8190`](../../../../../crates/renovate-core/src/util.rs#L8190) |
| 89 | distincts between different log statements | ported | [`crates/renovate-core/src/util.rs:8210`](../../../../../crates/renovate-core/src/util.rs#L8210) |
| 106 | parameters are taken into account when de-duplicating calls | ported | [`crates/renovate-core/src/util.rs:8235`](../../../../../crates/renovate-core/src/util.rs#L8235) |
| 124 | allows mixing single-time and regular logging | ported | [`crates/renovate-core/src/util.rs:8281`](../../../../../crates/renovate-core/src/util.rs#L8281) |
| 146 | supports reset method | ported | [`crates/renovate-core/src/util.rs:8309`](../../../../../crates/renovate-core/src/util.rs#L8309) |


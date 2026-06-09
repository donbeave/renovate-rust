# `lib/logger/once.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should call a function only once | ported | [`crates/renovate-core/src/util.rs:8133`](../../../../../crates/renovate-core/src/util.rs#L8133) |
| 28 | supports support distinct calls | ported | [`crates/renovate-core/src/util.rs:8144`](../../../../../crates/renovate-core/src/util.rs#L8144) |
| 44 | resets keys | ported | [`crates/renovate-core/src/util.rs:8161`](../../../../../crates/renovate-core/src/util.rs#L8161) |
| 60 | logs once per function call | ported | [`crates/renovate-core/src/util.rs:8172`](../../../../../crates/renovate-core/src/util.rs#L8172) |
| 73 | distincts between log levels | ported | [`crates/renovate-core/src/util.rs:8189`](../../../../../crates/renovate-core/src/util.rs#L8189) |
| 89 | distincts between different log statements | ported | [`crates/renovate-core/src/util.rs:8209`](../../../../../crates/renovate-core/src/util.rs#L8209) |
| 106 | parameters are taken into account when de-duplicating calls | ported | [`crates/renovate-core/src/util.rs:8234`](../../../../../crates/renovate-core/src/util.rs#L8234) |
| 124 | allows mixing single-time and regular logging | ported | [`crates/renovate-core/src/util.rs:8280`](../../../../../crates/renovate-core/src/util.rs#L8280) |
| 146 | supports reset method | ported | [`crates/renovate-core/src/util.rs:8308`](../../../../../crates/renovate-core/src/util.rs#L8308) |


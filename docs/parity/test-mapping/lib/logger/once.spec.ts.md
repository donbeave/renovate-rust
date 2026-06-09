# `lib/logger/once.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should call a function only once | ported | [`crates/renovate-core/src/util.rs:8229`](../../../../../crates/renovate-core/src/util.rs#L8229) |
| 28 | supports support distinct calls | ported | [`crates/renovate-core/src/util.rs:8240`](../../../../../crates/renovate-core/src/util.rs#L8240) |
| 44 | resets keys | ported | [`crates/renovate-core/src/util.rs:8257`](../../../../../crates/renovate-core/src/util.rs#L8257) |
| 60 | logs once per function call | ported | [`crates/renovate-core/src/util.rs:8268`](../../../../../crates/renovate-core/src/util.rs#L8268) |
| 73 | distincts between log levels | ported | [`crates/renovate-core/src/util.rs:8285`](../../../../../crates/renovate-core/src/util.rs#L8285) |
| 89 | distincts between different log statements | ported | [`crates/renovate-core/src/util.rs:8305`](../../../../../crates/renovate-core/src/util.rs#L8305) |
| 106 | parameters are taken into account when de-duplicating calls | ported | [`crates/renovate-core/src/util.rs:8330`](../../../../../crates/renovate-core/src/util.rs#L8330) |
| 124 | allows mixing single-time and regular logging | ported | [`crates/renovate-core/src/util.rs:8376`](../../../../../crates/renovate-core/src/util.rs#L8376) |
| 146 | supports reset method | ported | [`crates/renovate-core/src/util.rs:8404`](../../../../../crates/renovate-core/src/util.rs#L8404) |


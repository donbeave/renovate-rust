# `lib/logger/once.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**3/9 in-scope tests ported** (6 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should call a function only once | ported | [`crates/renovate-core/src/util.rs:6766`](../../../../../crates/renovate-core/src/util.rs#L6766) |
| 28 | supports support distinct calls | ported | [`crates/renovate-core/src/util.rs:6777`](../../../../../crates/renovate-core/src/util.rs#L6777) |
| 44 | resets keys | ported | [`crates/renovate-core/src/util.rs:6794`](../../../../../crates/renovate-core/src/util.rs#L6794) |
| 60 | logs once per function call | pending | — |
| 73 | distincts between log levels | pending | — |
| 89 | distincts between different log statements | pending | — |
| 106 | parameters are taken into account when de-duplicating calls | pending | — |
| 124 | allows mixing single-time and regular logging | pending | — |
| 146 | supports reset method | pending | — |


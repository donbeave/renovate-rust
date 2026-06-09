# `lib/util/string.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | replaceat inserts newstring which is one char longer than oldstring | ported | [`crates/renovate-core/src/util.rs:6625`](../../../../../crates/renovate-core/src/util.rs#L6625) |
| 22 | replaceat inserts newstring which is significantly longer than oldstring | ported | [`crates/renovate-core/src/util.rs:6633`](../../../../../crates/renovate-core/src/util.rs#L6633) |
| 35 | reverts to literal match if either is falsey | ported | [`crates/renovate-core/src/util.rs:6653`](../../../../../crates/renovate-core/src/util.rs#L6653) |
| 42 | coercestring | ported | [`crates/renovate-core/src/util.rs:6664`](../../../../../crates/renovate-core/src/util.rs#L6664) |
| 51 | _(it.each / template — verify manually)_ | ? | — |
| 81 | capitalizes | ported | [`crates/renovate-core/src/util.rs:6723`](../../../../../crates/renovate-core/src/util.rs#L6723) |


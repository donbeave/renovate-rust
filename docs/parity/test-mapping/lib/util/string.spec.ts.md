# `lib/util/string.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | replaceat inserts newstring which is one char longer than oldstring | ported | `crates/renovate-core/src/util.rs:5673` |
| 22 | replaceat inserts newstring which is significantly longer than oldstring | ported | `crates/renovate-core/src/util.rs:5681` |
| 35 | reverts to literal match if either is falsey | ported | `crates/renovate-core/src/util.rs:5689` |
| 42 | coercestring | ported | `crates/renovate-core/src/util.rs:5700` |
| 51 | _(it.each / template — verify manually)_ | ? | — |
| 81 | capitalizes | ported | `crates/renovate-core/src/util.rs:5759` |


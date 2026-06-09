# `lib/logger/utils.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | _(it.each / template — verify manually)_ | ? | — |
| 26 | sanitizes boxed string objects as strings | ported | [`crates/renovate-core/src/util.rs:7272`](../../../../../crates/renovate-core/src/util.rs#L7272) |
| 39 | preserves secret template strings in redacted fields | ported | [`crates/renovate-core/src/util.rs:7349`](../../../../../crates/renovate-core/src/util.rs#L7349) |
| 90 | preparezodissues | ported | [`crates/renovate-core/src/util.rs:7108`](../../../../../crates/renovate-core/src/util.rs#L7108) |
| 178 | prepareerror | ported | [`crates/renovate-core/src/util.rs:7288`](../../../../../crates/renovate-core/src/util.rs#L7288) |
| 203 | handles http timout error | ported | [`crates/renovate-core/src/util.rs:7160`](../../../../../crates/renovate-core/src/util.rs#L7160) |
| 219 | handles rawexec error | ported | [`crates/renovate-core/src/util.rs:7197`](../../../../../crates/renovate-core/src/util.rs#L7197) |
| 232 | handles aggregateerror | ported | [`crates/renovate-core/src/util.rs:7232`](../../../../../crates/renovate-core/src/util.rs#L7232) |


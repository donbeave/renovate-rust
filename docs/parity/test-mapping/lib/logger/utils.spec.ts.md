# `lib/logger/utils.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | _(it.each / template — verify manually)_ | ? | — |
| 26 | sanitizes boxed string objects as strings | ported | [`crates/renovate-core/src/util.rs:7179`](../../../../../crates/renovate-core/src/util.rs#L7179) |
| 39 | preserves secret template strings in redacted fields | ported | [`crates/renovate-core/src/util.rs:7256`](../../../../../crates/renovate-core/src/util.rs#L7256) |
| 90 | preparezodissues | ported | [`crates/renovate-core/src/util.rs:7015`](../../../../../crates/renovate-core/src/util.rs#L7015) |
| 178 | prepareerror | ported | [`crates/renovate-core/src/util.rs:7195`](../../../../../crates/renovate-core/src/util.rs#L7195) |
| 203 | handles http timout error | ported | [`crates/renovate-core/src/util.rs:7067`](../../../../../crates/renovate-core/src/util.rs#L7067) |
| 219 | handles rawexec error | ported | [`crates/renovate-core/src/util.rs:7104`](../../../../../crates/renovate-core/src/util.rs#L7104) |
| 232 | handles aggregateerror | ported | [`crates/renovate-core/src/util.rs:7139`](../../../../../crates/renovate-core/src/util.rs#L7139) |


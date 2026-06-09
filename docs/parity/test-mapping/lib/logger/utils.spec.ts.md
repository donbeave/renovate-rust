# `lib/logger/utils.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | _(it.each / template — verify manually)_ | ? | — |
| 26 | sanitizes boxed string objects as strings | ported | [`crates/renovate-core/src/util.rs:7176`](../../../../../crates/renovate-core/src/util.rs#L7176) |
| 39 | preserves secret template strings in redacted fields | ported | [`crates/renovate-core/src/util.rs:7253`](../../../../../crates/renovate-core/src/util.rs#L7253) |
| 90 | preparezodissues | ported | [`crates/renovate-core/src/util.rs:7012`](../../../../../crates/renovate-core/src/util.rs#L7012) |
| 178 | prepareerror | ported | [`crates/renovate-core/src/util.rs:7192`](../../../../../crates/renovate-core/src/util.rs#L7192) |
| 203 | handles http timout error | ported | [`crates/renovate-core/src/util.rs:7064`](../../../../../crates/renovate-core/src/util.rs#L7064) |
| 219 | handles rawexec error | ported | [`crates/renovate-core/src/util.rs:7101`](../../../../../crates/renovate-core/src/util.rs#L7101) |
| 232 | handles aggregateerror | ported | [`crates/renovate-core/src/util.rs:7136`](../../../../../crates/renovate-core/src/util.rs#L7136) |


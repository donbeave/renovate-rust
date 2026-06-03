# `lib/logger/utils.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**2/8 in-scope tests ported** (6 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | _(it.each / template — verify manually)_ | ? | — |
| 26 | sanitizes boxed string objects as strings | pending | — |
| 39 | preserves secret template strings in redacted fields | ported | [`crates/renovate-core/src/util.rs:5945`](../../../../../crates/renovate-core/src/util.rs#L5945) |
| 90 | preparezodissues | pending | — |
| 178 | prepareerror | pending | — |
| 203 | handles http timout error | pending | — |
| 219 | handles rawexec error | pending | — |
| 232 | handles aggregateerror | pending | — |


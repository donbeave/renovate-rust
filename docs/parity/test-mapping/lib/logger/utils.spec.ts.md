# `lib/logger/utils.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | _(it.each / template — verify manually)_ | ? | — |
| 26 | sanitizes boxed string objects as strings | ported | [`crates/renovate-core/src/util.rs:7175`](../../../../../crates/renovate-core/src/util.rs#L7175) |
| 39 | preserves secret template strings in redacted fields | ported | [`crates/renovate-core/src/util.rs:7252`](../../../../../crates/renovate-core/src/util.rs#L7252) |
| 90 | preparezodissues | ported | [`crates/renovate-core/src/util.rs:7011`](../../../../../crates/renovate-core/src/util.rs#L7011) |
| 178 | prepareerror | ported | [`crates/renovate-core/src/util.rs:7191`](../../../../../crates/renovate-core/src/util.rs#L7191) |
| 203 | handles http timout error | ported | [`crates/renovate-core/src/util.rs:7063`](../../../../../crates/renovate-core/src/util.rs#L7063) |
| 219 | handles rawexec error | ported | [`crates/renovate-core/src/util.rs:7100`](../../../../../crates/renovate-core/src/util.rs#L7100) |
| 232 | handles aggregateerror | ported | [`crates/renovate-core/src/util.rs:7135`](../../../../../crates/renovate-core/src/util.rs#L7135) |


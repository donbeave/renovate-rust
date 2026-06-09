# `lib/logger/remap.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | returns null if no remaps are set | ported | [`crates/renovate-core/src/util.rs:10038`](../../../../../crates/renovate-core/src/util.rs#L10038) |
| 24 | performs global remaps | ported | [`crates/renovate-core/src/util.rs:10044`](../../../../../crates/renovate-core/src/util.rs#L10044) |
| 33 | performs repository-level remaps | ported | [`crates/renovate-core/src/util.rs:10054`](../../../../../crates/renovate-core/src/util.rs#L10054) |
| 44 | prioritizes repository-level remaps over global remaps | ported | [`crates/renovate-core/src/util.rs:10061`](../../../../../crates/renovate-core/src/util.rs#L10061) |
| 55 | supports regex patterns | ported | [`crates/renovate-core/src/util.rs:10072`](../../../../../crates/renovate-core/src/util.rs#L10072) |
| 64 | does not match against invalid regex patterns | ported | [`crates/renovate-core/src/util.rs:10082`](../../../../../crates/renovate-core/src/util.rs#L10082) |


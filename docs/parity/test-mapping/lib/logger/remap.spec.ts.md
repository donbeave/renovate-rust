# `lib/logger/remap.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | returns null if no remaps are set | ported | [`crates/renovate-core/src/util.rs:10136`](../../../../../crates/renovate-core/src/util.rs#L10136) |
| 24 | performs global remaps | ported | [`crates/renovate-core/src/util.rs:10142`](../../../../../crates/renovate-core/src/util.rs#L10142) |
| 33 | performs repository-level remaps | ported | [`crates/renovate-core/src/util.rs:10152`](../../../../../crates/renovate-core/src/util.rs#L10152) |
| 44 | prioritizes repository-level remaps over global remaps | ported | [`crates/renovate-core/src/util.rs:10159`](../../../../../crates/renovate-core/src/util.rs#L10159) |
| 55 | supports regex patterns | ported | [`crates/renovate-core/src/util.rs:10170`](../../../../../crates/renovate-core/src/util.rs#L10170) |
| 64 | does not match against invalid regex patterns | ported | [`crates/renovate-core/src/util.rs:10180`](../../../../../crates/renovate-core/src/util.rs#L10180) |


# `lib/logger/remap.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | returns null if no remaps are set | ported | [`crates/renovate-core/src/util.rs:10041`](../../../../../crates/renovate-core/src/util.rs#L10041) |
| 24 | performs global remaps | ported | [`crates/renovate-core/src/util.rs:10047`](../../../../../crates/renovate-core/src/util.rs#L10047) |
| 33 | performs repository-level remaps | ported | [`crates/renovate-core/src/util.rs:10057`](../../../../../crates/renovate-core/src/util.rs#L10057) |
| 44 | prioritizes repository-level remaps over global remaps | ported | [`crates/renovate-core/src/util.rs:10064`](../../../../../crates/renovate-core/src/util.rs#L10064) |
| 55 | supports regex patterns | ported | [`crates/renovate-core/src/util.rs:10075`](../../../../../crates/renovate-core/src/util.rs#L10075) |
| 64 | does not match against invalid regex patterns | ported | [`crates/renovate-core/src/util.rs:10085`](../../../../../crates/renovate-core/src/util.rs#L10085) |


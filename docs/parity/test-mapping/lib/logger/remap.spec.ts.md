# `lib/logger/remap.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | returns null if no remaps are set | ported | [`crates/renovate-core/src/util.rs:10040`](../../../../../crates/renovate-core/src/util.rs#L10040) |
| 24 | performs global remaps | ported | [`crates/renovate-core/src/util.rs:10046`](../../../../../crates/renovate-core/src/util.rs#L10046) |
| 33 | performs repository-level remaps | ported | [`crates/renovate-core/src/util.rs:10056`](../../../../../crates/renovate-core/src/util.rs#L10056) |
| 44 | prioritizes repository-level remaps over global remaps | ported | [`crates/renovate-core/src/util.rs:10063`](../../../../../crates/renovate-core/src/util.rs#L10063) |
| 55 | supports regex patterns | ported | [`crates/renovate-core/src/util.rs:10074`](../../../../../crates/renovate-core/src/util.rs#L10074) |
| 64 | does not match against invalid regex patterns | ported | [`crates/renovate-core/src/util.rs:10084`](../../../../../crates/renovate-core/src/util.rs#L10084) |


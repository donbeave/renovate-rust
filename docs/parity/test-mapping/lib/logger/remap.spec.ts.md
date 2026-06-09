# `lib/logger/remap.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | returns null if no remaps are set | ported | [`crates/renovate-core/src/util.rs:10039`](../../../../../crates/renovate-core/src/util.rs#L10039) |
| 24 | performs global remaps | ported | [`crates/renovate-core/src/util.rs:10045`](../../../../../crates/renovate-core/src/util.rs#L10045) |
| 33 | performs repository-level remaps | ported | [`crates/renovate-core/src/util.rs:10055`](../../../../../crates/renovate-core/src/util.rs#L10055) |
| 44 | prioritizes repository-level remaps over global remaps | ported | [`crates/renovate-core/src/util.rs:10062`](../../../../../crates/renovate-core/src/util.rs#L10062) |
| 55 | supports regex patterns | ported | [`crates/renovate-core/src/util.rs:10073`](../../../../../crates/renovate-core/src/util.rs#L10073) |
| 64 | does not match against invalid regex patterns | ported | [`crates/renovate-core/src/util.rs:10083`](../../../../../crates/renovate-core/src/util.rs#L10083) |


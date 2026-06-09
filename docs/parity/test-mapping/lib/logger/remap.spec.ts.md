# `lib/logger/remap.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | returns null if no remaps are set | ported | [`crates/renovate-core/src/util.rs:10044`](../../../../../crates/renovate-core/src/util.rs#L10044) |
| 24 | performs global remaps | ported | [`crates/renovate-core/src/util.rs:10050`](../../../../../crates/renovate-core/src/util.rs#L10050) |
| 33 | performs repository-level remaps | ported | [`crates/renovate-core/src/util.rs:10060`](../../../../../crates/renovate-core/src/util.rs#L10060) |
| 44 | prioritizes repository-level remaps over global remaps | ported | [`crates/renovate-core/src/util.rs:10067`](../../../../../crates/renovate-core/src/util.rs#L10067) |
| 55 | supports regex patterns | ported | [`crates/renovate-core/src/util.rs:10078`](../../../../../crates/renovate-core/src/util.rs#L10078) |
| 64 | does not match against invalid regex patterns | ported | [`crates/renovate-core/src/util.rs:10088`](../../../../../crates/renovate-core/src/util.rs#L10088) |


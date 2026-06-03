# `lib/modules/manager/mise/schema.spec.ts`

[← `manager/mise`](../../../../_by-module/manager/mise.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | defaults tools to empty object when [tools] is absent | ported | [`crates/renovate-core/src/extractors/mise.rs:1734`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1734) |
| 13 | defaults tools to empty object for empty toml | ported | [`crates/renovate-core/src/extractors/mise.rs:1742`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1742) |
| 17 | parses [tools] when present | ported | [`crates/renovate-core/src/extractors/mise.rs:1750`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1750) |


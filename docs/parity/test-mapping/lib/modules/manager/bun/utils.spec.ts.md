# `lib/modules/manager/bun/utils.spec.ts`

[← `manager/bun`](../../../../_by-module/manager/bun.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | should return false when filename does not start with pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4793`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4793) |
| 14 | should correctly evaluate filename when it starts with pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4800`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4800) |
| 30 | should filter files matching workspaces and pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4807`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4807) |


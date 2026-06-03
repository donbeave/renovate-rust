# `lib/modules/manager/bun/utils.spec.ts`

[← `manager/bun`](../../../../_by-module/manager/bun.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | should return false when filename does not start with pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4749`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4749) |
| 14 | should correctly evaluate filename when it starts with pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4756`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4756) |
| 30 | should filter files matching workspaces and pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4763`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4763) |


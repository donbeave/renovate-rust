# `lib/modules/manager/bun/utils.spec.ts`

[← `manager/bun`](../../../../_by-module/manager/bun.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | should return false when filename does not start with pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4791`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4791) |
| 14 | should correctly evaluate filename when it starts with pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4798`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4798) |
| 30 | should filter files matching workspaces and pwd | ported | [`crates/renovate-core/src/extractors/npm.rs:4805`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4805) |


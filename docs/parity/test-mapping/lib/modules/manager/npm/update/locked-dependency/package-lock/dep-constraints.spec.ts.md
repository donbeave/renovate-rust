# `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5392`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5392) |
| 29 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5405`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5405) |
| 41 | skips non-matching direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5417`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5417) |
| 53 | finds direct devdependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5427`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5427) |


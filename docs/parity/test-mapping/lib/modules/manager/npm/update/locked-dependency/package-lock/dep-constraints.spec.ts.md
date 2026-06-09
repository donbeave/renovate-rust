# `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5395`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5395) |
| 29 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5408`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5408) |
| 41 | skips non-matching direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5420`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5420) |
| 53 | finds direct devdependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5430`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5430) |


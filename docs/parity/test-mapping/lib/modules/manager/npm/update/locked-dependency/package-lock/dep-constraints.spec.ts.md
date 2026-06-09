# `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5396`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5396) |
| 29 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5409`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5409) |
| 41 | skips non-matching direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5421`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5421) |
| 53 | finds direct devdependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5431`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5431) |


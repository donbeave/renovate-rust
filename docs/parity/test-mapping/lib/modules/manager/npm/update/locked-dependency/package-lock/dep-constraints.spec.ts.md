# `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5376`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5376) |
| 29 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5389`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5389) |
| 41 | skips non-matching direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5401`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5401) |
| 53 | finds direct devdependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5411`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5411) |


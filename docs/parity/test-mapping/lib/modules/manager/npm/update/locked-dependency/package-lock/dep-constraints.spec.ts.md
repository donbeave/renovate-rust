# `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5403`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5403) |
| 29 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5416`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5416) |
| 41 | skips non-matching direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5428`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5428) |
| 53 | finds direct devdependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5438`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5438) |


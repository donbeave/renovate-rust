# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5791`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5791) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5802`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5802) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5816`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5816) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5830`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5830) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5844`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5844) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5858`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5858) |


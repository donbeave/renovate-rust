# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5783`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5783) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5794`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5794) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5808`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5808) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5822`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5822) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5836`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5836) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5850`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5850) |


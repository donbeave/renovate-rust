# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5793`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5793) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5804`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5804) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5818`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5818) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5832`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5832) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5846`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5846) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5860`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5860) |


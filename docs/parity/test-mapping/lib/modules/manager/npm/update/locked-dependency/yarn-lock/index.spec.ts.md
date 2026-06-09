# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5792`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5792) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5803`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5803) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5817`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5817) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5831`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5831) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5845`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5845) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5859`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5859) |


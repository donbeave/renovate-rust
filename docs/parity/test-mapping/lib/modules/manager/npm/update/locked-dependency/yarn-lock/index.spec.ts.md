# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5790`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5790) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5801`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5801) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5815`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5815) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5829`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5829) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5843`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5843) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5857`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5857) |


# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5787`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5787) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5798`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5798) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5812`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5812) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5826`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5826) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5840`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5840) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5854`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5854) |


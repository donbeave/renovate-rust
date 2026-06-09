# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5770`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5770) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5781`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5781) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5795`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5795) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5809`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5809) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5823`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5823) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5837`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5837) |


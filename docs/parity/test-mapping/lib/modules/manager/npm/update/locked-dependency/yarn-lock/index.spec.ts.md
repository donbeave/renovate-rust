# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5797`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5797) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5808`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5808) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5822`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5822) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5836`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5836) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5850`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5850) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5864`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5864) |


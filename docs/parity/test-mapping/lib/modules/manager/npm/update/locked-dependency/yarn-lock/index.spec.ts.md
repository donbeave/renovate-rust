# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5786`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5786) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5797`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5797) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5811`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5811) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5825`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5825) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5839`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5839) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5853`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5853) |


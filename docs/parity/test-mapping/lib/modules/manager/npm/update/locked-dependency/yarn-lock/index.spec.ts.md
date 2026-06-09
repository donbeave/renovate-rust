# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5789`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5789) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5800`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5800) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5814`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5814) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5828`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5828) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5842`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5842) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5856`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5856) |


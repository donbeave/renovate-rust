# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | [`crates/renovate-core/src/extractors/npm.rs:5788`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5788) |
| 22 | returns if yarn lock 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:5799`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5799) |
| 30 | fails if cannot find dep | ported | [`crates/renovate-core/src/extractors/npm.rs:5813`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5813) |
| 38 | returns already-updated | ported | [`crates/renovate-core/src/extractors/npm.rs:5827`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5827) |
| 46 | fails if cannot update dep in-range | ported | [`crates/renovate-core/src/extractors/npm.rs:5841`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5841) |
| 54 | succeeds if can update within range | ported | [`crates/renovate-core/src/extractors/npm.rs:5855`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5855) |


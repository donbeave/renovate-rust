# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5341`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5341) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5350`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5350) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5365`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5365) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5263`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5263) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5213`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5213) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5235`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5235) |


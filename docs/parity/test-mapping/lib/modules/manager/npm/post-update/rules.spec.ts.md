# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5343`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5343) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5352`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5352) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5367`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5367) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5265`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5265) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5215`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5215) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5237`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5237) |


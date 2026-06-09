# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5336`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5336) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5345`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5345) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5360`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5360) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5258`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5258) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5208`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5208) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5230`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5230) |


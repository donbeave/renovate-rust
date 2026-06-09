# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5340`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5340) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5349`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5349) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5364`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5364) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5262`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5262) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5212`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5212) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5234`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5234) |


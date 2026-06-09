# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5335`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5335) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5344`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5344) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5359`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5359) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5257`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5257) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5207`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5207) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5229`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5229) |


# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5320`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5320) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5329`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5329) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5344`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5344) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5242`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5242) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5192`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5192) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5214`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5214) |


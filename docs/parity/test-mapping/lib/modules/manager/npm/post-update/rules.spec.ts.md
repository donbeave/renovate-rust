# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5342`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5342) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5351`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5351) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5366`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5366) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5264`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5264) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5214`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5214) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5236`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5236) |


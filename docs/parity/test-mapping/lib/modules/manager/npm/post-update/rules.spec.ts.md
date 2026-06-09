# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5339`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5339) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5348`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5348) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5363`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5363) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5261`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5261) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5211`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5211) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5233`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5233) |


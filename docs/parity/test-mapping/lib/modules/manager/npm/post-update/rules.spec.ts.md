# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5333`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5333) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5342`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5342) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5357`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5357) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5255`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5255) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5205`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5205) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5227`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5227) |


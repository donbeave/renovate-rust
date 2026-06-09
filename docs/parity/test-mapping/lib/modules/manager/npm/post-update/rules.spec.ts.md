# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5347`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5347) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5356`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5356) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5371`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5371) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5269`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5269) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5219`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5219) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5241`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5241) |


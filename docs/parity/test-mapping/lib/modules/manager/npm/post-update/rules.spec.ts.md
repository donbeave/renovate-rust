# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | [`crates/renovate-core/src/extractors/npm.rs:5337`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5337) |
| 16 | returns empty if no resolvedhost | ported | [`crates/renovate-core/src/extractors/npm.rs:5346`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5346) |
| 23 | returns rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5361`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5361) |
| 64 | returns mixed rules content | ported | [`crates/renovate-core/src/extractors/npm.rs:5259`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5259) |
| 146 | uses rules without host type | ported | [`crates/renovate-core/src/extractors/npm.rs:5209`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5209) |
| 167 | deduplicates host rules while prefering npm type ones | ported | [`crates/renovate-core/src/extractors/npm.rs:5231`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5231) |


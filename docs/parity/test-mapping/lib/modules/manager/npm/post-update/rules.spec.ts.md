# `lib/modules/manager/npm/post-update/rules.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns empty if no rules | ported | `crates/renovate-core/src/extractors/npm.rs:5293` |
| 16 | returns empty if no resolvedhost | ported | `crates/renovate-core/src/extractors/npm.rs:5302` |
| 23 | returns rules content | ported | `crates/renovate-core/src/extractors/npm.rs:5317` |
| 64 | returns mixed rules content | ported | `crates/renovate-core/src/extractors/npm.rs:5215` |
| 146 | uses rules without host type | ported | `crates/renovate-core/src/extractors/npm.rs:5165` |
| 167 | deduplicates host rules while prefering npm type ones | ported | `crates/renovate-core/src/extractors/npm.rs:5187` |


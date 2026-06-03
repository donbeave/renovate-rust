# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 17 | returns if cannot parse lock file | ported | `crates/renovate-core/src/extractors/npm.rs:5743` |
| 22 | returns if yarn lock 2 | ported | `crates/renovate-core/src/extractors/npm.rs:5754` |
| 30 | fails if cannot find dep | ported | `crates/renovate-core/src/extractors/npm.rs:5768` |
| 38 | returns already-updated | ported | `crates/renovate-core/src/extractors/npm.rs:5782` |
| 46 | fails if cannot update dep in-range | ported | `crates/renovate-core/src/extractors/npm.rs:5796` |
| 54 | succeeds if can update within range | ported | `crates/renovate-core/src/extractors/npm.rs:5810` |


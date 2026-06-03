# `lib/modules/manager/npm/range.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | returns same if not auto | ported | `crates/renovate-core/src/extractors/npm.rs:4688` |
| 10 | widens peerdependencies | ported | `crates/renovate-core/src/extractors/npm.rs:4694` |
| 18 | widens complex ranges | ported | `crates/renovate-core/src/extractors/npm.rs:4701` |
| 27 | widens complex bump | ported | `crates/renovate-core/src/extractors/npm.rs:4708` |
| 36 | defaults to update-lockfile | ported | `crates/renovate-core/src/extractors/npm.rs:4715` |


# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | `crates/renovate-core/src/extractors/npm.rs:3431` |
| 15 | extracts | ported | `crates/renovate-core/src/extractors/npm.rs:3438` |
| 33 | extracts npm 7 lockfile | ported | `crates/renovate-core/src/extractors/npm.rs:3470` |
| 51 | extracts npm 9 lockfile | ported | `crates/renovate-core/src/extractors/npm.rs:3497` |
| 69 | returns null if no deps | ported | `crates/renovate-core/src/extractors/npm.rs:3529` |
| 75 | returns null on read error | ported | `crates/renovate-core/src/extractors/npm.rs:3536` |


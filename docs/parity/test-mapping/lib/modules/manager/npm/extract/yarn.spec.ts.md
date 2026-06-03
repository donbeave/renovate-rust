# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | `crates/renovate-core/src/extractors/npm.rs:3543` |
| 20 | extracts yarn 1 | ported | `crates/renovate-core/src/extractors/npm.rs:3552` |
| 30 | extracts yarn 2 | ported | `crates/renovate-core/src/extractors/npm.rs:3590` |
| 40 | extracts yarn 2 cache version | ported | `crates/renovate-core/src/extractors/npm.rs:3631` |
| 50 | ignores individual invalid entries | ported | `crates/renovate-core/src/extractors/npm.rs:3673` |
| 63 | getyarnversionfromlock | ported | `crates/renovate-core/src/extractors/npm.rs:3697` |
| 83 | handles empty catalog entries | ported | `crates/renovate-core/src/extractors/npm.rs:3750` |
| 91 | parses valid .yarnrc.yml file | ported | `crates/renovate-core/src/extractors/npm.rs:3757` |
| 133 | finds relevant lockfile | ported | `crates/renovate-core/src/extractors/npm.rs:3788` |


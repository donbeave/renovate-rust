# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3592`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3592) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3601`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3601) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3639`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3639) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3680`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3680) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3722`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3722) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3746`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3746) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3799`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3799) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3806`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3806) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3837`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3837) |


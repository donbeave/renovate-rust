# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3593`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3593) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3602`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3602) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3640`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3640) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3681`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3681) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3723`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3723) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3747`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3747) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3800`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3800) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3807`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3807) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3838`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3838) |


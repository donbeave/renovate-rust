# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3589`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3589) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3598`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3598) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3636`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3636) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3677`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3677) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3719`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3719) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3743`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3743) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3796`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3796) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3803`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3803) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3834`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3834) |


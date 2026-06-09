# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3586`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3586) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3595`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3595) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3633`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3633) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3674`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3674) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3716`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3716) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3740`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3740) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3793`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3793) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3800`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3800) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3831`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3831) |


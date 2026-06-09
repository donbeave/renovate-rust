# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3590`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3590) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3599`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3599) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3637`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3637) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3678`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3678) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3720`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3720) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3744`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3744) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3797`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3797) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3804`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3804) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3835`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3835) |


# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3597`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3597) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3606`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3606) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3644`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3644) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3685`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3685) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3727`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3727) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3751`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3751) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3804`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3804) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3811`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3811) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3842`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3842) |


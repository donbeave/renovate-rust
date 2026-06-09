# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3588`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3588) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3597`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3597) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3635`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3635) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3676`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3676) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3718`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3718) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3742`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3742) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3795`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3795) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3802`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3802) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3833`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3833) |


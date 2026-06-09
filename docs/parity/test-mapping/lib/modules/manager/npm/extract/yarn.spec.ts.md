# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3587`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3587) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3596`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3596) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3634`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3634) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3675`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3675) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3717`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3717) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3741`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3741) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3794`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3794) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3801`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3801) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3832`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3832) |


# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3583`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3583) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3592`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3592) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3630`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3630) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3671`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3671) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3713`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3713) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3737`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3737) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3790`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3790) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3797`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3797) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3828`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3828) |


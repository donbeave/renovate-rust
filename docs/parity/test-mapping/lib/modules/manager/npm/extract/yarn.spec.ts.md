# `lib/modules/manager/npm/extract/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns empty if exception parsing | ported | [`crates/renovate-core/src/extractors/npm.rs:3585`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3585) |
| 20 | extracts yarn 1 | ported | [`crates/renovate-core/src/extractors/npm.rs:3594`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3594) |
| 30 | extracts yarn 2 | ported | [`crates/renovate-core/src/extractors/npm.rs:3632`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3632) |
| 40 | extracts yarn 2 cache version | ported | [`crates/renovate-core/src/extractors/npm.rs:3673`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3673) |
| 50 | ignores individual invalid entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3715`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3715) |
| 63 | getyarnversionfromlock | ported | [`crates/renovate-core/src/extractors/npm.rs:3739`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3739) |
| 83 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3792`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3792) |
| 91 | parses valid .yarnrc.yml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3799`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3799) |
| 133 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3830`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3830) |


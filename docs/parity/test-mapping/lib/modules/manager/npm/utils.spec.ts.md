# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4946`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4946) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4959`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4959) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4967`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4967) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4990`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4990) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5003`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5003) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5030`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5030) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5038`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5038) |


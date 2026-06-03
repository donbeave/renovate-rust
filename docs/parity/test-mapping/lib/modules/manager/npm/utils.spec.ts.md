# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4903`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4903) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4916`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4916) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4924`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4924) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4947`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4947) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:4960`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4960) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:4987`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4987) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:4995`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4995) |


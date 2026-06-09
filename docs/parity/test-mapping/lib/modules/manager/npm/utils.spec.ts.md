# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4947`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4947) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4960`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4960) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4968`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4968) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4991`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4991) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5004`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5004) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5031`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5031) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5039`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5039) |


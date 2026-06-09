# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4949`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4949) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4962`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4962) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4970`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4970) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4993`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4993) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5006`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5006) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5033`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5033) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5041`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5041) |


# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4951`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4951) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4964`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4964) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4972`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4972) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4995`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4995) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5008`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5008) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5035`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5035) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5043`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5043) |


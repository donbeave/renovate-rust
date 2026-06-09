# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4950`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4950) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4963`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4963) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4971`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4971) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4994`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4994) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5007`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5007) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5034`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5034) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5042`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5042) |


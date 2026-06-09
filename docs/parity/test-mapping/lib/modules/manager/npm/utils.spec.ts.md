# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4948`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4948) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4961`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4961) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4969`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4969) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4992`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4992) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5005`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5005) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5032`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5032) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5040`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5040) |


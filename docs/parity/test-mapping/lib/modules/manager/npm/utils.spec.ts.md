# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4952`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4952) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4965`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4965) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4973`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4973) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4996`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4996) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5009`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5009) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5036`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5036) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5044`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5044) |


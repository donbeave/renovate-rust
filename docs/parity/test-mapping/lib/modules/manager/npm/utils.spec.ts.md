# `lib/modules/manager/npm/utils.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4953`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4953) |
| 37 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/npm.rs:4966`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4966) |
| 48 | composes lockfile string out of an object | ported | [`crates/renovate-core/src/extractors/npm.rs:4974`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4974) |
| 66 | adds trailing newline to match npms behavior and avoid diffs | ported | [`crates/renovate-core/src/extractors/npm.rs:4997`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4997) |
| 81 | loads and parses package.json correctly | ported | [`crates/renovate-core/src/extractors/npm.rs:5010`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5010) |
| 100 | returns empty object when package.json is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:5037`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5037) |
| 105 | returns empty object when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:5045`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5045) |


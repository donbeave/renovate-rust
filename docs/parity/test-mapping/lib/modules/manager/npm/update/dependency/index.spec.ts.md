# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6004`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6004) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6022`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6022) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6039`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6039) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6057`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6057) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6074`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6074) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6095`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6095) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6110`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6110) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6125`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6125) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6143`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6143) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6159`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6159) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6175`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6175) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6188`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6188) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6201`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6201) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6215`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6215) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6228`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6228) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6245`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6245) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6261`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6261) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6277`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6277) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6314`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6314) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6341`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6341) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6364`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6364) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6395`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6395) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6426`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6426) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6893`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6893) |


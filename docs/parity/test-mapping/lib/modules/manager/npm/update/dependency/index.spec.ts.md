# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6013`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6013) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6031`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6031) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6048`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6048) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6066`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6066) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6083`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6083) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6104`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6104) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6119`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6119) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6134`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6134) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6152`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6152) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6168`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6168) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6184`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6184) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6197`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6197) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6210`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6210) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6224`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6224) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6237`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6237) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6254`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6254) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6270`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6270) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6286`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6286) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6323`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6323) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6350`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6350) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6373`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6373) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6404`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6404) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6435`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6435) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6902`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6902) |


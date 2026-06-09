# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6009`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6009) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6027`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6027) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6044`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6044) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6062`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6062) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6079`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6079) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6100`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6100) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6115`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6115) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6130`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6130) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6148`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6148) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6164`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6164) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6180`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6180) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6193`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6193) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6206`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6206) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6220`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6220) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6233`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6233) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6250`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6250) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6266`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6266) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6282`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6282) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6319`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6319) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6346`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6346) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6369`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6369) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6400`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6400) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6431`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6431) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6898`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6898) |


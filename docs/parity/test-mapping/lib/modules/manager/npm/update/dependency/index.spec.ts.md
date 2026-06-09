# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6002`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6002) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6020`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6020) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6037`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6037) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6055`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6055) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6072`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6072) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6093`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6093) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6108`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6108) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6123`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6123) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6141`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6141) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6157`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6157) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6173`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6173) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6186`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6186) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6199`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6199) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6213`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6213) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6226`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6226) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6243`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6243) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6259`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6259) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6275`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6275) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6312`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6312) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6339`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6339) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6362`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6362) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6393`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6393) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6424`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6424) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6891`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6891) |


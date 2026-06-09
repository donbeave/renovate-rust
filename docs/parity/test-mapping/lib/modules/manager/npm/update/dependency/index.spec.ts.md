# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:5999`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5999) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6017`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6017) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6034`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6034) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6052`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6052) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6069`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6069) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6090`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6090) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6105`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6105) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6120`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6120) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6138`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6138) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6154`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6154) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6170`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6170) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6183`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6183) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6196`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6196) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6210`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6210) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6223`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6223) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6240`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6240) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6256`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6256) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6272`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6272) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6309`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6309) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6336`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6336) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6359`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6359) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6390`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6390) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6421`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6421) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6888`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6888) |


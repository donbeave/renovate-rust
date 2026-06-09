# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6007`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6007) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6025`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6025) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6042`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6042) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6060`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6060) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6077`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6077) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6098`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6098) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6113`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6113) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6128`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6128) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6146`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6146) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6162`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6162) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6178`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6178) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6191`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6191) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6204`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6204) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6218`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6218) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6231`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6231) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6248`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6248) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6264`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6264) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6280`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6280) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6317`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6317) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6344`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6344) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6367`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6367) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6398`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6398) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6429`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6429) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6896`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6896) |


# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6003`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6003) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6021`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6021) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6038`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6038) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6056`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6056) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6073`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6073) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6094`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6094) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6109`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6109) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6124`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6124) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6142`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6142) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6158`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6158) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6174`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6174) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6187`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6187) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6200`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6200) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6214`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6214) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6227`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6227) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6244`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6244) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6260`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6260) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6276`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6276) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6313`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6313) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6340`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6340) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6363`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6363) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6394`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6394) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6425`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6425) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6892`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6892) |


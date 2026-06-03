# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:5959`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5959) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:5977`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5977) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:5994`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5994) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6012`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6012) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6029`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6029) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6050`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6050) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6065`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6065) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6080`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6080) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6098`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6098) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6114`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6114) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6130`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6130) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6143`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6143) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6156`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6156) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6170`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6170) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6183`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6183) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6200`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6200) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6216`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6216) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6232`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6232) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6269`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6269) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6296`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6296) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6319`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6319) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6350`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6350) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6381`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6381) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6848`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6848) |


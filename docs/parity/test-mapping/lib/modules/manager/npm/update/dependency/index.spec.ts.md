# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6001`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6001) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6019`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6019) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6036`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6036) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6054`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6054) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6071`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6071) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6092`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6092) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6107`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6107) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6122`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6122) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6140`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6140) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6156`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6156) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6172`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6172) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6185`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6185) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6198`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6198) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6212`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6212) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6225`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6225) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6242`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6242) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6258`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6258) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6274`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6274) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6311`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6311) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6338`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6338) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6361`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6361) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6392`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6392) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6423`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6423) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6890`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6890) |


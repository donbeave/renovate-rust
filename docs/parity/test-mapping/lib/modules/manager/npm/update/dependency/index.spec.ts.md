# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:5986`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5986) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6004`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6004) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6021`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6021) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6039`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6039) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6056`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6056) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6077`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6077) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6092`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6092) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6107`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6107) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6125`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6125) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6141`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6141) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6157`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6157) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6170`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6170) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6183`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6183) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6197`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6197) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6210`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6210) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6227`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6227) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6243`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6243) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6259`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6259) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6296`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6296) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6323`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6323) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6346`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6346) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6377`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6377) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6408`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6408) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6875`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6875) |


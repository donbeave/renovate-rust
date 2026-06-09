# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6005`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6005) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6023`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6023) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6040`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6040) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6058`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6058) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6075`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6075) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6096`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6096) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6111`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6111) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6126`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6126) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6144`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6144) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6160`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6160) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6176`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6176) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6189`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6189) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6202`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6202) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6216`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6216) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6229`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6229) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6246`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6246) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6262`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6262) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6278`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6278) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6315`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6315) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6342`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6342) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6365`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6365) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6396`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6396) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6427`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6427) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6894`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6894) |


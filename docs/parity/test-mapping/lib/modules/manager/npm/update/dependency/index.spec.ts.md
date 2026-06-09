# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6008`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6008) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6026`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6026) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6043`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6043) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6061`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6061) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6078`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6078) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6099`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6099) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6114`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6114) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6129`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6129) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6147`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6147) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6163`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6163) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6179`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6179) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6192`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6192) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6205`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6205) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6219`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6219) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6232`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6232) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6249`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6249) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6265`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6265) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6281`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6281) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6318`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6318) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6345`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6345) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6368`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6368) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6399`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6399) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6430`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6430) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6897`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6897) |


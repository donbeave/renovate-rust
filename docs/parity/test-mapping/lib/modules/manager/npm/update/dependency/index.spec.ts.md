# `lib/modules/manager/npm/update/dependency/index.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | replaces a dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6006`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6006) |
| 28 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6024`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6024) |
| 52 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6041`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6041) |
| 77 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6059`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6059) |
| 101 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6076`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6076) |
| 123 | updates resolutions too | ported | [`crates/renovate-core/src/extractors/npm.rs:6097`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6097) |
| 138 | updates glob resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6112`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6112) |
| 153 | updates glob resolutions without dep | ported | [`crates/renovate-core/src/extractors/npm.rs:6127`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6127) |
| 170 | replaces only the first instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6145`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6145) |
| 185 | replaces only the second instance of a value | ported | [`crates/renovate-core/src/extractors/npm.rs:6161`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6161) |
| 200 | handles the case where the desired version is already supported | ported | [`crates/renovate-core/src/extractors/npm.rs:6177`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6177) |
| 214 | returns null if throws error | ported | [`crates/renovate-core/src/extractors/npm.rs:6190`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6190) |
| 228 | updates packagemanager | ported | [`crates/renovate-core/src/extractors/npm.rs:6203`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6203) |
| 243 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6217`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6217) |
| 257 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6230`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6230) |
| 273 | supports alias-based replacement | ported | [`crates/renovate-core/src/extractors/npm.rs:6247`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6247) |
| 291 | replaces glob package resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6263`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6263) |
| 307 | pins also the version in patch with npm protocol in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6279`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6279) |
| 322 | replaces also the version in patch with range in resolutions | ported | [`crates/renovate-core/src/extractors/npm.rs:6316`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6316) |
| 337 | handles override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6343`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6343) |
| 361 | handles override dependency object | ported | [`crates/renovate-core/src/extractors/npm.rs:6366`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6366) |
| 390 | handles override dependency object where lastparent === depname | ported | [`crates/renovate-core/src/extractors/npm.rs:6397`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6397) |
| 419 | handles pnpm.override dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6428`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6428) |
| 446 | handles yarn.catalogs dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:6895`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6895) |


# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6912`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6912) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6927`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6927) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6941`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6941) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6955`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6955) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6968`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6968) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6983`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6983) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7001`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7001) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7015`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7015) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7032`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7032) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7052`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7052) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7073`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7073) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7093`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7093) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7110`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7110) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7125`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7125) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7139`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7139) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7152`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7152) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7166`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7166) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7180`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7180) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7196`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7196) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7210`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7210) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7226`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7226) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7240`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7240) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7254`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7254) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7269`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7269) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7283`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7283) |


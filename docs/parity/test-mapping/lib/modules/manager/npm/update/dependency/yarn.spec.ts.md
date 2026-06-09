# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6910`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6910) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6925`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6925) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6939`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6939) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6953`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6953) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6966`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6966) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6981`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6981) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6999`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6999) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7013`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7013) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7030`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7030) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7050`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7050) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7071`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7071) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7091`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7091) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7108`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7108) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7123`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7123) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7137`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7137) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7150`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7150) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7164`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7164) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7178`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7178) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7194`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7194) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7208`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7208) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7224`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7224) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7238`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7238) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7252`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7252) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7267`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7267) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7281`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7281) |


# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6918`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6918) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6933`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6933) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6947`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6947) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6961`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6961) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6974`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6974) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6989`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6989) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7007`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7007) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7021`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7021) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7038`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7038) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7058`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7058) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7079`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7079) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7099`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7099) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7116`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7116) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7131`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7131) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7145`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7145) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7158`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7158) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7172`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7172) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7186`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7186) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7202`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7202) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7216`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7216) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7232`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7232) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7246`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7246) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7260`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7260) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7275`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7275) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7289`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7289) |


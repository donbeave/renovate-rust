# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6917`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6917) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6932`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6932) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6946`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6946) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6960`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6960) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6973`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6973) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6988`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6988) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7006`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7006) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7020`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7020) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7037`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7037) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7057`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7057) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7078`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7078) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7098`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7098) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7115`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7115) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7130`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7130) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7144`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7144) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7157`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7157) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7171`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7171) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7185`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7185) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7201`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7201) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7215`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7215) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7231`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7231) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7245`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7245) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7259`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7259) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7274`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7274) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7288`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7288) |


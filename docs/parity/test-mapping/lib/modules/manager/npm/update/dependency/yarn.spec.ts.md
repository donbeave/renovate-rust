# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6916`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6916) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6931`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6931) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6945`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6945) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6959`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6959) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6972`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6972) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6987`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6987) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7005`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7005) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7019`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7019) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7036`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7036) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7056`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7056) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7077`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7077) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7097`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7097) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7114`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7114) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7129`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7129) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7143`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7143) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7156`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7156) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7170`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7170) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7184`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7184) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7200`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7200) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7214`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7214) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7230`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7230) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7244`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7244) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7258`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7258) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7273`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7273) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7287`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7287) |


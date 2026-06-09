# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6913`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6913) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6928`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6928) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6942`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6942) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6956`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6956) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6969`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6969) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6984`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6984) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7002`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7002) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7016`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7016) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7033`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7033) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7053`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7053) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7074`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7074) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7094`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7094) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7111`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7111) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7126`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7126) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7140`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7140) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7153`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7153) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7167`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7167) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7181`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7181) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7197`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7197) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7211`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7211) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7227`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7227) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7241`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7241) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7255`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7255) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7270`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7270) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7284`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7284) |


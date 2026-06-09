# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6915`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6915) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6930`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6930) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6944`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6944) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6958`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6958) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6971`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6971) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6986`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6986) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7004`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7004) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7018`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7018) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7035`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7035) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7055`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7055) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7076`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7076) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7096`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7096) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7113`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7113) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7128`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7128) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7142`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7142) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7155`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7155) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7169`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7169) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7183`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7183) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7199`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7199) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7213`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7213) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7229`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7229) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7243`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7243) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7257`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7257) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7272`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7272) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7286`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7286) |


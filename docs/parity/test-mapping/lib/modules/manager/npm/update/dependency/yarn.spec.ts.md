# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6922`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6922) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6937`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6937) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6951`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6951) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6965`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6965) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6978`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6978) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6993`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6993) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7011`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7011) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7025`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7025) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7042`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7042) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7062`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7062) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7083`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7083) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7103`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7103) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7120`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7120) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7135`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7135) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7149`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7149) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7162`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7162) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7176`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7176) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7190`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7190) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7206`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7206) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7220`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7220) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7236`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7236) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7250`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7250) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7264`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7264) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7279`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7279) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7293`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7293) |


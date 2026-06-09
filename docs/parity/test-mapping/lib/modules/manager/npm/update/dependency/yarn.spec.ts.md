# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6914`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6914) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6929`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6929) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6943`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6943) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6957`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6957) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6970`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6970) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6985`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6985) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7003`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7003) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7017`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7017) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7034`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7034) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7054`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7054) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7075`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7075) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7095`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7095) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7112`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7112) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7127`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7127) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7141`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7141) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7154`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7154) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7168`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7168) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7182`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7182) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7198`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7198) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7212`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7212) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7228`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7228) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7242`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7242) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7256`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7256) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7271`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7271) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7285`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7285) |


# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/25 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6911`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6911) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6926`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6926) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6940`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6940) |
| 78 | ensure trace logging | opt-out | asserts TypeScript logger spy behavior (exact logger.logger.trace call with the formatted message 'npm.updateYarnrcCatalogDependency(): yarn.catalog.default::default.react = 19.0.0') on successful catalog update; the core business logic of updateYarnrcCatalogDependency is already covered by other ported tests in the same spec; no direct Rust logger spy equivalent without altering production instrumentation (Rust uses tracing/debug macros) or test harness |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6954`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6954) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6967`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6967) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6982`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6982) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:7000`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7000) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:7014`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7014) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7031`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7031) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7051`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7051) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7072`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7072) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7092`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7092) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7109`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7109) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7124`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7124) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7138`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7138) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7151`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7151) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7165`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7165) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7179`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7179) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7195`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7195) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7209`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7209) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7225`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7225) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7239`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7239) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7253`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7253) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7268`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7268) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7282`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7282) |


# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/26 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | [`crates/renovate-core/src/extractors/npm.rs:6895`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6895) |
| 33 | ensure continuation even if catalog list and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6910`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6910) |
| 55 | ensure continuation even if dependency and update does not match | ported | [`crates/renovate-core/src/extractors/npm.rs:6924`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6924) |
| 78 | ensure trace logging | pending | — |
| 103 | returns null if catalogname is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6938`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6938) |
| 125 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6951`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6951) |
| 150 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6966`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6966) |
| 177 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6984`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6984) |
| 197 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6998`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6998) |
| 224 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:7015`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7015) |
| 251 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:7035`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7035) |
| 279 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:7056`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7056) |
| 306 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:7076`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7076) |
| 334 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:7093`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7093) |
| 357 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:7108`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7108) |
| 375 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:7122`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7122) |
| 389 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:7135`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7135) |
| 415 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7149`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7149) |
| 440 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:7163`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7163) |
| 469 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:7179`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7179) |
| 494 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:7193`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7193) |
| 524 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7209`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7209) |
| 549 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:7223`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7223) |
| 574 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:7237`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7237) |
| 603 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:7252`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7252) |
| 630 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:7266`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7266) |


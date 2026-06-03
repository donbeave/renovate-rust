# `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**25/26 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns null if catalogname is missing and logs error | ported | `crates/renovate-core/src/extractors/npm.rs:6868` |
| 33 | ensure continuation even if catalog list and update does not match | ported | `crates/renovate-core/src/extractors/npm.rs:6883` |
| 55 | ensure continuation even if dependency and update does not match | ported | `crates/renovate-core/src/extractors/npm.rs:6897` |
| 78 | ensure trace logging | pending | — |
| 103 | returns null if catalogname is missing | ported | `crates/renovate-core/src/extractors/npm.rs:6911` |
| 125 | handles implicit default catalog dependency | ported | `crates/renovate-core/src/extractors/npm.rs:6924` |
| 150 | handles explicit named catalog dependency | ported | `crates/renovate-core/src/extractors/npm.rs:6939` |
| 177 | does nothing if the new and old values match | ported | `crates/renovate-core/src/extractors/npm.rs:6957` |
| 197 | replaces package | ported | `crates/renovate-core/src/extractors/npm.rs:6971` |
| 224 | replaces a github dependency value | ported | `crates/renovate-core/src/extractors/npm.rs:6988` |
| 251 | replaces a npm package alias | ported | `crates/renovate-core/src/extractors/npm.rs:7008` |
| 279 | replaces a github short hash | ported | `crates/renovate-core/src/extractors/npm.rs:7029` |
| 306 | replaces a github fully specified version | ported | `crates/renovate-core/src/extractors/npm.rs:7049` |
| 334 | returns null if the dependency is not present in the target catalog | ported | `crates/renovate-core/src/extractors/npm.rs:7066` |
| 357 | returns null if catalogs are missing | ported | `crates/renovate-core/src/extractors/npm.rs:7081` |
| 375 | returns null if empty file | ported | `crates/renovate-core/src/extractors/npm.rs:7095` |
| 389 | preserves literal whitespace | ported | `crates/renovate-core/src/extractors/npm.rs:7108` |
| 415 | preserves single quote style | ported | `crates/renovate-core/src/extractors/npm.rs:7122` |
| 440 | preserves comments | ported | `crates/renovate-core/src/extractors/npm.rs:7136` |
| 469 | preserves double quote style | ported | `crates/renovate-core/src/extractors/npm.rs:7152` |
| 494 | preserves anchors, replacing only the value | ported | `crates/renovate-core/src/extractors/npm.rs:7166` |
| 524 | preserves whitespace with anchors | ported | `crates/renovate-core/src/extractors/npm.rs:7182` |
| 549 | preserves quotation style with anchors | ported | `crates/renovate-core/src/extractors/npm.rs:7196` |
| 574 | preserves formatting in flow style syntax | ported | `crates/renovate-core/src/extractors/npm.rs:7210` |
| 603 | does not replace aliases in the value position | ported | `crates/renovate-core/src/extractors/npm.rs:7225` |
| 630 | does not replace aliases in the key position | ported | `crates/renovate-core/src/extractors/npm.rs:7239` |


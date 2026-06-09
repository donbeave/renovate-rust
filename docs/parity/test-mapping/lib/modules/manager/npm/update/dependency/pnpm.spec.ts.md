# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6501`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6501) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6515`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6515) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6532`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6532) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6550`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6550) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6571`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6571) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6585`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6585) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6602`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6602) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6622`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6622) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6643`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6643) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6663`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6663) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6680`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6680) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6694`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6694) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6708`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6708) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6721`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6721) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6736`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6736) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6750`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6750) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6768`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6768) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6782`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6782) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6800`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6800) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6814`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6814) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6828`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6828) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6843`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6843) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6858`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6858) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6875`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6875) |


# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6454`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6454) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6468`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6468) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6485`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6485) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6503`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6503) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6524`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6524) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6538`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6538) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6555`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6555) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6575`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6575) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6596`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6596) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6616`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6616) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6633`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6633) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6647`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6647) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6661`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6661) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6674`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6674) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6689`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6689) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6703`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6703) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6721`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6721) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6735`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6735) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6753`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6753) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6767`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6767) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6781`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6781) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6796`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6796) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6811`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6811) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6828`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6828) |


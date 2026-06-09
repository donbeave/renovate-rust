# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6496`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6496) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6510`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6510) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6527`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6527) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6545`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6545) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6566`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6566) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6580`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6580) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6597`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6597) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6617`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6617) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6638`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6638) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6658`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6658) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6675`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6675) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6689`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6689) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6703`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6703) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6716`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6716) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6731`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6731) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6745`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6745) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6763`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6763) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6777`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6777) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6795`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6795) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6809`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6809) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6823`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6823) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6838`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6838) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6853`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6853) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6870`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6870) |


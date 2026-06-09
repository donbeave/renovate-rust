# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6481`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6481) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6495`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6495) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6512`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6512) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6530`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6530) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6551`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6551) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6565`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6565) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6582`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6582) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6602`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6602) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6623`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6623) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6643`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6643) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6660`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6660) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6674`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6674) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6688`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6688) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6701`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6701) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6716`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6716) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6730`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6730) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6748`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6748) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6762`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6762) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6780`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6780) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6794`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6794) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6808`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6808) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6823`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6823) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6838`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6838) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6855`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6855) |


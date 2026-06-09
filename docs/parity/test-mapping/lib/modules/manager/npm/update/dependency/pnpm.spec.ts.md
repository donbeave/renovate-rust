# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6498`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6498) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6512`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6512) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6529`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6529) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6547`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6547) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6568`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6568) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6582`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6582) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6599`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6599) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6619`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6619) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6640`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6640) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6660`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6660) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6677`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6677) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6691`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6691) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6705`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6705) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6718`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6718) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6733`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6733) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6747`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6747) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6765`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6765) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6779`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6779) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6797`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6797) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6811`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6811) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6825`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6825) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6840`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6840) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6855`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6855) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6872`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6872) |


# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6497`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6497) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6511`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6511) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6528`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6528) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6546`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6546) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6567`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6567) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6581`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6581) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6598`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6598) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6618`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6618) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6639`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6639) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6659`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6659) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6676`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6676) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6690`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6690) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6704`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6704) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6717`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6717) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6732`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6732) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6746`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6746) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6764`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6764) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6778`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6778) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6796`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6796) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6810`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6810) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6824`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6824) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6839`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6839) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6854`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6854) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6871`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6871) |


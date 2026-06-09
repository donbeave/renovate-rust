# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6502`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6502) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6516`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6516) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6533`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6533) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6551`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6551) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6572`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6572) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6586`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6586) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6603`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6603) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6623`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6623) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6644`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6644) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6664`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6664) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6681`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6681) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6695`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6695) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6709`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6709) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6722`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6722) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6737`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6737) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6751`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6751) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6769`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6769) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6783`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6783) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6801`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6801) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6815`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6815) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6829`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6829) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6844`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6844) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6859`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6859) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6876`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6876) |


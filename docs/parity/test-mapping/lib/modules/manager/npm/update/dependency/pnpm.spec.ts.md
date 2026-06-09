# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6503`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6503) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6517`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6517) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6534`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6534) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6552`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6552) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6573`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6573) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6587`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6587) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6604`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6604) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6624`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6624) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6645`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6645) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6665`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6665) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6682`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6682) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6696`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6696) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6710`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6710) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6723`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6723) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6738`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6738) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6752`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6752) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6770`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6770) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6784`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6784) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6802`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6802) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6816`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6816) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6830`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6830) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6845`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6845) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6860`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6860) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6877`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6877) |


# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6504`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6504) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6518`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6518) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6535`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6535) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6553`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6553) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6574`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6574) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6588`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6588) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6605`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6605) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6625`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6625) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6646`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6646) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6666`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6666) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6683`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6683) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6697`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6697) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6711`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6711) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6724`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6724) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6739`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6739) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6753`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6753) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6771`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6771) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6785`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6785) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6803`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6803) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6817`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6817) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6831`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6831) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6846`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6846) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6861`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6861) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6878`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6878) |


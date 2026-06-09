# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6508`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6508) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6522`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6522) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6539`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6539) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6557`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6557) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6578`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6578) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6592`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6592) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6609`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6609) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6629`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6629) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6650`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6650) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6670`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6670) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6687`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6687) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6701`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6701) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6715`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6715) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6728`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6728) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6743`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6743) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6757`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6757) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6775`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6775) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6789`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6789) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6807`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6807) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6821`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6821) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6835`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6835) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6850`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6850) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6865`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6865) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6882`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6882) |


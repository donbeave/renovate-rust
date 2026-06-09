# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6494`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6494) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6508`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6508) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6525`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6525) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6543`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6543) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6564`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6564) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6578`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6578) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6595`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6595) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6615`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6615) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6636`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6636) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6656`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6656) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6673`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6673) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6687`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6687) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6701`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6701) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6714`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6714) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6729`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6729) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6743`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6743) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6761`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6761) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6775`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6775) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6793`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6793) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6807`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6807) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6821`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6821) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6836`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6836) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6851`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6851) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6868`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6868) |


# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6499`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6499) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6513`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6513) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6530`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6530) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6548`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6548) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6569`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6569) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6583`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6583) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6600`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6600) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6620`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6620) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6641`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6641) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6661`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6661) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6678`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6678) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6692`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6692) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6706`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6706) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6719`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6719) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6734`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6734) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6748`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6748) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6766`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6766) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6780`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6780) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6798`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6798) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6812`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6812) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6826`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6826) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6841`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6841) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6856`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6856) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6873`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6873) |


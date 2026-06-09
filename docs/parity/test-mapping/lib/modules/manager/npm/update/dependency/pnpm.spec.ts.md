# `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null on invalid input | ported | [`crates/renovate-core/src/extractors/npm.rs:6500`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6500) |
| 19 | handles implicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6514`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6514) |
| 46 | handles explicit default catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6531`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6531) |
| 75 | handles explicit named catalog dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:6549`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6549) |
| 111 | does nothing if the new and old values match | ported | [`crates/renovate-core/src/extractors/npm.rs:6570`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6570) |
| 132 | replaces package | ported | [`crates/renovate-core/src/extractors/npm.rs:6584`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6584) |
| 160 | replaces a github dependency value | ported | [`crates/renovate-core/src/extractors/npm.rs:6601`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6601) |
| 189 | replaces a npm package alias | ported | [`crates/renovate-core/src/extractors/npm.rs:6621`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6621) |
| 219 | replaces a github short hash | ported | [`crates/renovate-core/src/extractors/npm.rs:6642`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6642) |
| 248 | replaces a github fully specified version | ported | [`crates/renovate-core/src/extractors/npm.rs:6662`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6662) |
| 277 | returns null if the dependency is not present in the target catalog | ported | [`crates/renovate-core/src/extractors/npm.rs:6679`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6679) |
| 298 | returns null if catalogs are missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6693`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6693) |
| 316 | returns null if empty file | ported | [`crates/renovate-core/src/extractors/npm.rs:6707`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6707) |
| 330 | preserves literal whitespace | ported | [`crates/renovate-core/src/extractors/npm.rs:6720`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6720) |
| 357 | preserves single quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6735`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6735) |
| 384 | preserves comments | ported | [`crates/renovate-core/src/extractors/npm.rs:6749`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6749) |
| 415 | preserves double quote style | ported | [`crates/renovate-core/src/extractors/npm.rs:6767`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6767) |
| 442 | preserves anchors, replacing only the value | ported | [`crates/renovate-core/src/extractors/npm.rs:6781`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6781) |
| 474 | preserves whitespace with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6799`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6799) |
| 501 | preserves quotation style with anchors | ported | [`crates/renovate-core/src/extractors/npm.rs:6813`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6813) |
| 528 | preserves formatting in flow style syntax | ported | [`crates/renovate-core/src/extractors/npm.rs:6827`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6827) |
| 559 | does not replace aliases in the value position | ported | [`crates/renovate-core/src/extractors/npm.rs:6842`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6842) |
| 587 | does not replace aliases in the key position | ported | [`crates/renovate-core/src/extractors/npm.rs:6857`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6857) |
| 611 | handles workspace overrides | ported | [`crates/renovate-core/src/extractors/npm.rs:6874`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6874) |


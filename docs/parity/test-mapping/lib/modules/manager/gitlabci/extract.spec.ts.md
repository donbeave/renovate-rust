# `lib/modules/manager/gitlabci/extract.spec.ts`

[← `manager/gitlabci`](../../../../_by-module/manager/gitlabci.md) · [all modules](../../../../README.md)

**14/14 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 22 | extracts from empty file | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:770`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L770) |
| 28 | returns null for empty | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:1172`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L1172) |
| 36 | extracts from multidoc yaml | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:785`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L785) |
| 46 | extracts multiple included image lines | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:1233`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L1233) |
| 57 | extracts named services | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:749`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L749) |
| 66 | extracts multiple named services | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:1178`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L1178) |
| 75 | extracts multiple image lines | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:724`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L724) |
| 94 | extracts multiple image lines with comments | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:793`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L793) |
| 110 | catches errors | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:1224`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L1224) |
| 118 | skips images with variables | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:776`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L776) |
| 172 | extract images from dependency proxy | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:816`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L816) |
| 229 | extract images via registry aliases | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:838`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L838) |
| 299 | extracts component references via registry aliases | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:890`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L890) |
| 377 | extracts component references | ported | [`crates/renovate-core/src/extractors/gitlabci.rs:946`](../../../../../../../crates/renovate-core/src/extractors/gitlabci.rs#L946) |


# `lib/modules/manager/terraform/extract.spec.ts`

[← `manager/terraform`](../../../../_by-module/manager/terraform.md) · [all modules](../../../../README.md)

**18/18 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 39 | returns null for empty | ported | [`crates/renovate-core/src/extractors/terraform.rs:2732`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2732) |
| 43 | returns null for no deps | ported | [`crates/renovate-core/src/extractors/terraform.rs:2838`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2838) |
| 54 | extracts modules | ported | [`crates/renovate-core/src/extractors/terraform.rs:1907`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L1907) |
| 221 | extracts bitbucket modules | ported | [`crates/renovate-core/src/extractors/terraform.rs:1975`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L1975) |
| 306 | extracts azuredevops modules | ported | [`crates/renovate-core/src/extractors/terraform.rs:2038`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2038) |
| 338 | resolves oci registry aliases | ported | [`crates/renovate-core/src/extractors/terraform.rs:2067`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2067) |
| 358 | handles invalid oci source url | ported | [`crates/renovate-core/src/extractors/terraform.rs:2088`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2088) |
| 374 | extracts oci modules and providers | ported | [`crates/renovate-core/src/extractors/terraform.rs:2103`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2103) |
| 463 | extracts providers | ported | [`crates/renovate-core/src/extractors/terraform.rs:1853`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L1853) |
| 579 | extracts docker resources | ported | [`crates/renovate-core/src/extractors/terraform.rs:2187`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2187) |
| 655 | extracts kubernetes resources | ported | [`crates/renovate-core/src/extractors/terraform.rs:2391`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2391) |
| 756 | returns dep with skipreason local | ported | [`crates/renovate-core/src/extractors/terraform.rs:1943`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L1943) |
| 767 | returns null with only not added resources | ported | [`crates/renovate-core/src/extractors/terraform.rs:2822`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2822) |
| 776 | extract helm releases | ported | [`crates/renovate-core/src/extractors/terraform.rs:2283`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2283) |
| 845 | update lockfile constraints with range strategy update-lockfile | ported | [`crates/renovate-core/src/extractors/terraform.rs:2756`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2756) |
| 884 | test terraform block with only requirement_terraform_version | ported | [`crates/renovate-core/src/extractors/terraform.rs:2848`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2848) |
| 904 | extracts terraform_version for tfe_workspace and ignores missing terraform_version keys | ported | [`crates/renovate-core/src/extractors/terraform.rs:2860`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2860) |
| 933 | return null if invalid hcl file | ported | [`crates/renovate-core/src/extractors/terraform.rs:2830`](../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L2830) |


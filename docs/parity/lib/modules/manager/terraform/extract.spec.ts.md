# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/extract.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 18 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 39 | ported | `terraform.rs` | `empty_file_returns_empty` | — |
| returns null for no deps | 43 | ported | `terraform.rs` | `data_block_not_extracted` | — |
| extracts  modules | 54 | ported | `terraform.rs` | `module_with_version` (+ module_without_version_skipped, module_with_git_source_skipped, mixed_providers_and_modules) | — |
| extracts bitbucket modules | 221 | ported | `terraform.rs` | `bitbucket_module_sources_are_extracted` | — |
| extracts azureDevOps modules | 306 | ported | `terraform.rs` | `azure_devops_module_sources_are_extracted` | — |
| resolves OCI registry aliases | 338 | ported | `terraform.rs` | `oci_module_registry_alias_is_applied` | — |
| handles invalid OCI source URL | 358 | ported | `terraform.rs` | `invalid_oci_module_source_has_skip_reason` | — |
| extracts OCI modules and providers | 374 | ported | `terraform.rs` | `oci_modules_and_required_providers_are_extracted` | — |
| extracts providers | 463 | ported | `terraform.rs` | `required_providers_block_form` (+ required_providers_inline_string_form, comments_ignored, provider_without_source_uses_name) | — |
| extracts docker resources | 579 | ported | `terraform.rs` | `docker_resources_are_extracted` | — |
| extracts kubernetes resources | 655 | ported | `terraform.rs` | `kubernetes_resources_are_extracted` | — |
| returns dep with skipReason local | 756 | ported | `terraform.rs` | `module_with_local_path_skipped` (+ local_module_has_skip_reason) | — |
| returns null with only not added resources | 767 | ported | `terraform.rs` | `resource_block_not_extracted` | — |
| extract helm releases | 776 | ported | `terraform.rs` | `helm_releases_are_extracted` | — |
| update lockfile constraints with range strategy update-lockfile | 845 | ported | `terraform.rs` | `provider_lockfile_versions_are_applied` | — |
| test terraform block with only requirement_terraform_version | 884 | ported | `terraform.rs` | `required_version_extracted_as_hashicorp_terraform` | — |
| extracts terraform_version for tfe_workspace and ignores missing terraform_version keys | 904 | ported | `terraform.rs` | `tfe_workspace_terraform_versions_are_extracted` | — |
| return null if invalid HCL file | 933 | ported | `terraform.rs` | `invalid_hcl_returns_empty` | — |

---


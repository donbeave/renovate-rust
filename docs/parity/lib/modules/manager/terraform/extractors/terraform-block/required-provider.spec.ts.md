# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/extractors/terraform-block/required-provider.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/extractors/terraform-block/required-provider.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty array if no terraform block is found | 8 | ported | `terraform.rs` | `required_provider_extract_empty_content_returns_no_deps` | — |
| return empty array if no required_providers block is found | 13 | ported | `terraform.rs` | `required_provider_extract_terraform_block_without_required_providers_returns_empty` | — |
| extract provider with version and registry url | 18 | ported | `terraform.rs` | `required_provider_extract_with_version_and_registry_url` | — |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-module/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-module/utils.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `modules/datasource/terraform-module/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns URL with relative SD for modules | 7 | ported | `datasources/terraform.rs` | `create_sd_backend_url_relative_modules` | — |
| returns URL with relative SD for providers | 21 | ported | `datasources/terraform.rs` | `create_sd_backend_url_relative_providers` | — |
| returns URL with absolute SD  for modules | 35 | ported | `datasources/terraform.rs` | `create_sd_backend_url_absolute_modules` | — |
| returns URL with absolute SD for providers and missing trailing slash | 49 | ported | `datasources/terraform.rs` | `create_sd_backend_url_absolute_no_trailing_slash` | — |
| returns URL with with empty SD | 63 | ported | `datasources/terraform.rs` | `create_sd_backend_url_empty_sd` | — |
| returns URL with with missing SD | 75 | ported | `datasources/terraform.rs` | `create_sd_backend_url_missing_sd` | — |
| uses the configured registry URL for standard package names | 87 | ported | `datasources/terraform.rs` | `get_registry_repository_standard` | — |
| extracts the registry from packageName when it is embedded | 99 | ported | `datasources/terraform.rs` | `get_registry_repository_embedded` | — |
| normalizes an embedded registry without a scheme | 111 | ported | `datasources/terraform.rs` | `get_registry_repository_no_scheme` | — |

---


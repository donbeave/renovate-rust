# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-module/base.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-module/base.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/terraform-module/base`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws ExternalHostError for EAI_AGAIN errors | 7 | not-applicable | — | — | datasource not implemented — terraform-module is a separate datasource for Terraform module registry; not implemented in Rust; tests verify ExternalHostError throwing for EAI_AGAIN and 503 errors |
| throws ExternalHostError for HTTP 503 errors | 22 | not-applicable | — | — | datasource not implemented — terraform-module is a separate datasource for Terraform module registry; not implemented in Rust; tests verify ExternalHostError throwing for EAI_AGAIN and 503 errors |

---

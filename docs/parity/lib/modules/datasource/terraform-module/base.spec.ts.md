# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-module/base.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-module/base.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/terraform-module/base`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws ExternalHostError for EAI_AGAIN errors | 7 | not-applicable | — | — | Renovate's Terraform module datasource external-host-error classification is not implemented in Rust; Rust latest-module lookup uses the shared HTTP error type. |
| throws ExternalHostError for HTTP 503 errors | 22 | not-applicable | — | — | Renovate's Terraform module datasource external-host-error classification is not implemented in Rust; Rust latest-module lookup returns `None` for non-success HTTP statuses. |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-module/base.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-module/base.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 2 | **Status:** partial

### `modules/datasource/terraform-module/base`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws ExternalHostError for EAI_AGAIN errors | 7 | not-applicable | — | — | Requires httpMock for EAI_AGAIN simulation; ExternalHostError is Node.js-specific error type |
| throws ExternalHostError for HTTP 503 errors | 22 | not-applicable | — | — | Requires httpMock for HTTP 503 simulation |

---

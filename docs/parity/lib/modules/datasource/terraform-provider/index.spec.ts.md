# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-provider/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-provider/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** done

### `modules/datasource/terraform-provider/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when both default registries return $description | 68 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes real data | 87 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when a third-party registry returns $description | 136 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes real data from third party | 156 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes data with alternative backend | 184 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes real data from OpenTofu registry docs API | 219 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns an empty release list for OpenTofu registry without versions | 253 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws for empty result | 274 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for non hashicorp dependency and releases.hashicorp.com registryUrl | 290 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| works for hashicorp dependency and releases.hashicorp.com | 299 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws for hashicorp dependency and releases.hashicorp.com 500 | 312 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| rethrows external-host-error for hashicorp dependency and releases.hashicorp.com | 325 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws if service discovery error | 338 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws if a version is requested which is not available | 352 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes real data | 367 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws if the retrieval of a single build fails | 447 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| can fetch zip hashes | 487 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not hard fail when the ziphashes endpoint is not available | 511 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---

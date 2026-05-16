# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-provider/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-provider/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/terraform-provider/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when both default registries return $description | 68 | not-applicable | — | — | Renovate's Terraform provider `getReleases` error/null matrix is not implemented in Rust; Rust exposes latest-version lookup only. |
| processes real data | 87 | not-applicable | — | — | Renovate's Terraform provider full release-list and metadata mapping are not implemented in Rust; Rust exposes latest-version lookup only. |
| returns null when a third-party registry returns $description | 136 | not-applicable | — | — | Renovate's Terraform provider third-party registry error/null matrix is not implemented in Rust. |
| processes real data from third party | 156 | not-applicable | — | — | Renovate's Terraform provider service discovery and third-party registry release-list contract are not implemented in Rust. |
| processes data with alternative backend | 184 | not-applicable | — | — | Renovate's Terraform provider alternative backend/service-discovery contract is not implemented in Rust. |
| processes real data from OpenTofu registry docs API | 219 | not-applicable | — | — | Renovate's OpenTofu provider registry docs API support is not implemented in Rust. |
| returns an empty release list for OpenTofu registry without versions | 253 | not-applicable | — | — | Renovate's OpenTofu provider registry docs API support is not implemented in Rust. |
| throws for empty result | 274 | not-applicable | — | — | Renovate's Terraform provider `getReleases` empty-result error contract is not implemented in Rust. |
| returns null for non hashicorp dependency and releases.hashicorp.com registryUrl | 290 | not-applicable | — | — | Renovate's releases.hashicorp.com provider fallback logic is not implemented in Rust. |
| works for hashicorp dependency and releases.hashicorp.com | 299 | not-applicable | — | — | Renovate's releases.hashicorp.com provider fallback logic is not implemented in Rust. |
| throws for hashicorp dependency and releases.hashicorp.com 500 | 312 | not-applicable | — | — | Renovate's releases.hashicorp.com provider fallback error handling is not implemented in Rust. |
| rethrows external-host-error for hashicorp dependency and releases.hashicorp.com | 325 | not-applicable | — | — | Renovate's releases.hashicorp.com provider fallback error handling is not implemented in Rust. |
| throws if service discovery error | 338 | not-applicable | — | — | Renovate's Terraform provider service discovery flow is not implemented in Rust. |
| throws if a version is requested which is not available | 352 | not-applicable | — | — | Renovate's Terraform provider single-version build lookup is not implemented in Rust. |
| processes real data | 367 | not-applicable | — | — | Renovate's Terraform provider single-version build metadata mapping is not implemented in Rust. |
| throws if the retrieval of a single build fails | 447 | not-applicable | — | — | Renovate's Terraform provider single-version build lookup is not implemented in Rust. |
| can fetch zip hashes | 487 | not-applicable | — | — | Renovate's Terraform provider zip hash endpoint support is not implemented in Rust. |
| does not hard fail when the ziphashes endpoint is not available | 511 | not-applicable | — | — | Renovate's Terraform provider zip hash endpoint support is not implemented in Rust. |

---


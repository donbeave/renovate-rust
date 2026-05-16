# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/terraform-module/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/terraform-module/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/terraform-module/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for the default registry when the module endpoint returns $description | 81 | not-applicable | — | — | Renovate's Terraform module `getReleases` error/null matrix is not implemented in Rust; Rust exposes latest-version lookup only. |
| returns releases, homepage, and source URL from the default registry | 100 | not-applicable | — | — | Renovate's Terraform module full release-list, homepage, and sourceUrl mapping are not implemented in Rust; Rust exposes latest-version lookup only. |
| returns null for a third-party registry when the versions endpoint returns $description | 127 | not-applicable | — | — | Renovate's Terraform module third-party registry error/null matrix is not implemented in Rust. |
| returns releases from a third-party registry | 147 | not-applicable | — | — | Renovate's Terraform module service discovery and third-party registry release-list contract are not implemented in Rust. |
| returns sourceUrl when a third-party registry includes one | 169 | not-applicable | — | — | Renovate's Terraform module service discovery, third-party registry sourceUrl mapping, and release-list contract are not implemented in Rust. |
| uses the registry embedded in packageName | 199 | not-applicable | — | — | Renovate's Terraform module embedded-registry parsing and service-discovery URL routing are not implemented in Rust. |
| uses the v1 extended endpoint for Terraform Cloud | 226 | not-applicable | — | — | Renovate's Terraform Cloud extended module endpoint is not implemented in Rust. |
| returns null when the third-party versions response has no modules | 260 | not-applicable | — | — | Renovate's Terraform module third-party registry response validation is not implemented in Rust. |
| returns null when service discovery fails | 277 | not-applicable | — | — | Renovate's Terraform module service discovery flow is not implemented in Rust. |
| uses the service discovery modules path when the registry serves a custom subpath | 290 | not-applicable | — | — | Renovate's Terraform module service discovery custom-path routing is not implemented in Rust. |
| processes real data from OpenTofu registry docs API | 313 | not-applicable | — | — | Renovate's OpenTofu registry docs API support is not implemented in Rust. |
| returns an empty release list for OpenTofu registry without versions | 348 | not-applicable | — | — | Renovate's OpenTofu registry docs API support is not implemented in Rust. |

---


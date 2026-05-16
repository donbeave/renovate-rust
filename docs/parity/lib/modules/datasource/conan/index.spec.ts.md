# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/conan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/conan/index.spec.ts
**Total tests:** 22 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/conan/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles package without digest | 38 | not-applicable | — | — | Renovate's Conan lockfile package/digest lookup contract is not implemented in Rust; Rust only exposes Conan Center latest-version lookup from `config.yml`. |
| handles digest | 43 | not-applicable | — | — | Renovate's Conan lockfile package/digest lookup contract is not implemented in Rust; Rust only exposes Conan Center latest-version lookup from `config.yml`. |
| returns null for missing revision | 56 | not-applicable | — | — | Renovate's Conan lockfile revision handling is not implemented in Rust. |
| handles bad return | 69 | not-applicable | — | — | Renovate's Conan datasource null-on-bad-response contract is not implemented in Rust. |
| handles empty return | 82 | not-applicable | — | — | Renovate's Conan datasource null-on-empty-response contract is not implemented in Rust. |
| handles bad registries | 95 | not-applicable | — | — | Renovate's Conan registry validation and fallback contract is not implemented in Rust. |
| handles missing packages | 109 | not-applicable | — | — | Renovate's Conan missing-package null contract is not implemented in Rust. |
| processes real versioned data | 122 | not-applicable | — | — | Renovate's Conan release-list response mapping is not implemented in Rust; Rust only returns a latest-version update summary. |
| processes mixed case names | 154 | not-applicable | — | — | Renovate's Conan mixed-case package release-list handling is not implemented in Rust. |
| uses github instead of conan center | 180 | not-applicable | — | — | Renovate's Conan GitHub source URL fallback is not implemented in Rust. |
| works with empty releases | 221 | not-applicable | — | — | Renovate's Conan empty-release-list contract is not implemented in Rust. |
| rejects userAndChannel for Conan Center | 237 | not-applicable | — | — | Renovate's Conan Center user/channel validation is not implemented in Rust. |
| handles mismatched userAndChannel versioned data | 247 | not-applicable | — | — | Renovate's Conan user/channel validation and release-list mapping are not implemented in Rust. |
| handles malformed packages | 261 | not-applicable | — | — | Renovate's Conan package schema validation is not implemented in Rust. |
| handles non 404 errors | 282 | not-applicable | — | — | Renovate's Conan non-404 error contract is not implemented in Rust. |
| handles missing slash on registries | 297 | not-applicable | — | — | Renovate's Conan registry URL normalization is not implemented in Rust. |
| artifactory sourceurl | 312 | not-applicable | — | — | Renovate's Conan Artifactory source URL extraction is not implemented in Rust. |
| artifactory header without api | 367 | not-applicable | — | — | Renovate's Conan Artifactory header parsing is not implemented in Rust. |
| artifactory invalid version | 398 | not-applicable | — | — | Renovate's Conan Artifactory version validation is not implemented in Rust. |
| non artifactory header | 425 | not-applicable | — | — | Renovate's Conan Artifactory header parsing is not implemented in Rust. |
| artifactory no package url | 442 | not-applicable | — | — | Renovate's Conan Artifactory package URL handling is not implemented in Rust. |
| artifactory http error | 492 | not-applicable | — | — | Renovate's Conan Artifactory HTTP error handling is not implemented in Rust. |

---


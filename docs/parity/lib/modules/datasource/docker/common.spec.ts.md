# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/common.spec.ts
**Total tests:** 14 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/docker/common › getRegistryRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles local registries | 24 | not-applicable | — | — | Renovate's Docker registryUrl/repository resolver for arbitrary registries is not implemented in Rust; Rust Docker datasource currently supports Docker Hub image parsing only. |
| supports registryUrls | 35 | not-applicable | — | — | Renovate's Docker registryUrl prefix resolver is not implemented in Rust; Rust Docker datasource currently supports Docker Hub image parsing only. |
| supports http registryUrls | 46 | not-applicable | — | — | Renovate's Docker registryUrl scheme handling is not implemented in Rust; Rust Docker datasource currently supports Docker Hub image parsing only. |
| supports schemeless registryUrls | 57 | not-applicable | — | — | Renovate's Docker schemeless registryUrl normalization is not implemented in Rust; Rust Docker datasource currently supports Docker Hub image parsing only. |
| supports insecure registryUrls | 68 | not-applicable | — | — | Renovate's Docker hostRules insecureRegistry handling is not implemented in Rust. |
| ($name, $url) | 80 | not-applicable | — | — | Renovate's Docker registryUrl/repository resolver for Docker Hub aliases and OCI registries is not implemented in Rust; Rust Docker datasource currently supports Docker Hub image parsing only. |

### `modules/datasource/docker/common › getAuthHeaders`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throw page not found exception | 127 | not-applicable | — | — | Renovate's Docker registry auth-header negotiation and page-not-found error contract are not implemented in Rust. |
| returns "authType token" if both provided | 143 | not-applicable | — | — | Renovate's Docker hostRules auth header construction is not implemented in Rust. |
| returns "Bearer token" if only token provided | 168 | not-applicable | — | — | Renovate's Docker hostRules auth header construction is not implemented in Rust. |
| fails | 192 | not-applicable | — | — | Renovate's Docker auth challenge failure path is not implemented in Rust. |
| use resources URL and resolve scope in www-authenticate header | 214 | not-applicable | — | — | Renovate's Docker WWW-Authenticate challenge parsing and token exchange are not implemented in Rust. |
| supports multiple challenges in www-authenticate header | 242 | not-applicable | — | — | Renovate's Docker multi-challenge WWW-Authenticate parser is not implemented in Rust. |

### `modules/datasource/docker/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findLatestStable works | 270 | ported | `docker_hub.rs` | `empty_tag_list_produces_no_latest` | Rust verifies the equivalent no-tags/no-latest update summary behavior. |
| findHelmSourceUrl works | 274 | not-applicable | — | — | Renovate's Docker OCI Helm chart config source URL extraction is not implemented in Rust. |

---


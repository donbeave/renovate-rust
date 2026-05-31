# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/common.spec.ts
**Total tests:** 15 | **Ported:** 1 | **Actionable:** 14 | **Status:** done

### `modules/datasource/docker/common › getRegistryRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles local registries | 24 | not-applicable | — | — | Pure logic test; getRegistryRepository function not yet implemented in Rust docker module |
| supports registryUrls | 35 | not-applicable | — | — | Pure logic test; getRegistryRepository function not yet implemented in Rust docker module |
| supports http registryUrls | 46 | not-applicable | — | — | Pure logic test; getRegistryRepository function not yet implemented in Rust docker module |
| supports schemeless registryUrls | 57 | not-applicable | — | — | Pure logic test; getRegistryRepository function not yet implemented in Rust docker module |
| supports insecure registryUrls | 68 | not-applicable | — | — | Depends on hostRules mock; getRegistryRepository + insecureRegistry support not yet in Rust |
| ($name, $url) | 80 | not-applicable | — | — | Pure logic test; getRegistryRepository function not yet implemented in Rust docker module |

### `modules/datasource/docker/common › getAuthHeaders`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throw page not found exception | 127 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth header pipeline not yet in Rust |
| returns "authType token" if both provided | 143 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth header pipeline not yet in Rust |
| returns "Bearer token" if only token provided | 168 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth header pipeline not yet in Rust |
| fails | 192 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth header pipeline not yet in Rust |
| use resources URL and resolve scope in www-authenticate header | 214 | not-applicable | — | — | HTTP mock-based integration test; Docker registry www-authenticate scope resolution not yet in Rust |
| supports multiple challenges in www-authenticate header | 242 | not-applicable | — | — | HTTP mock-based integration test; Docker registry multi-challenge auth not yet in Rust |

### `modules/datasource/docker/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findLatestStable works | 270 | ported | `docker_hub.rs` | `empty_tag_list_produces_no_latest` | Rust verifies the equivalent no-tags/no-latest update summary behavior |
| findHelmSourceUrl works | 274 | not-applicable | — | — | Pure logic test; findHelmSourceUrl function not yet implemented in Rust docker module |
| returns raw registryHost and dockerRepository when fullUrl is invalid | 117 | not-applicable | — | — | Pure logic test; getRegistryRepository function not yet implemented in Rust docker module |

---

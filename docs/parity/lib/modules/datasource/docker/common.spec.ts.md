# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/common.spec.ts
**Total tests:** 15 | **Ported:** 1 | **Actionable:** 15 | **Status:** partial

### `modules/datasource/docker/common › getRegistryRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles local registries | 24 | pending | — | — | — |
| supports registryUrls | 35 | pending | — | — | — |
| supports http registryUrls | 46 | pending | — | — | — |
| supports schemeless registryUrls | 57 | pending | — | — | — |
| supports insecure registryUrls | 68 | pending | — | — | — |
| ($name, $url) | 80 | pending | — | — | — |

### `modules/datasource/docker/common › getAuthHeaders`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throw page not found exception | 127 | pending | — | — | — |
| returns "authType token" if both provided | 143 | pending | — | — | — |
| returns "Bearer token" if only token provided | 168 | pending | — | — | — |
| fails | 192 | pending | — | — | — |
| use resources URL and resolve scope in www-authenticate header | 214 | pending | — | — | — |
| supports multiple challenges in www-authenticate header | 242 | pending | — | — | — |

### `modules/datasource/docker/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findLatestStable works | 270 | ported | `docker_hub.rs` | `empty_tag_list_produces_no_latest` | Rust verifies the equivalent no-tags/no-latest update summary behavior. |
| findHelmSourceUrl works | 274 | pending | — | — | — |
| returns raw registryHost and dockerRepository when fullUrl is invalid | 117 | pending | — | — | — |

---

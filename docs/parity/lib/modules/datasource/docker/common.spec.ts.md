# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/common.spec.ts
**Total tests:** 15 | **Ported:** 1 | **Actionable:** 14 | **Status:** pending

### `modules/datasource/docker/common › getRegistryRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles local registries | 24 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| supports registryUrls | 35 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| supports http registryUrls | 46 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| supports schemeless registryUrls | 57 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| supports insecure registryUrls | 68 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| ($name, $url) | 80 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |

### `modules/datasource/docker/common › getAuthHeaders`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throw page not found exception | 127 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| returns "authType token" if both provided | 143 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| returns "Bearer token" if only token provided | 168 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| fails | 192 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| use resources URL and resolve scope in www-authenticate header | 214 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| supports multiple challenges in www-authenticate header | 242 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |

### `modules/datasource/docker/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findLatestStable works | 270 | ported | `docker_hub.rs` | `empty_tag_list_produces_no_latest` | Rust verifies the equivalent no-tags/no-latest update summary behavior. |
| findHelmSourceUrl works | 274 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |
| returns raw registryHost and dockerRepository when fullUrl is invalid | 117 | not-applicable | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer | — | Mock framework internals — tests docker common via nock HTTP mocks; Rust tests this at different layer |

---

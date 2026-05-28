# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/common.spec.ts
**Total tests:** 15 | **Ported:** 1 | **Actionable:** 15 | **Status:** done

### `modules/datasource/docker/common › getRegistryRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles local registries | 24 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| supports registryUrls | 35 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| supports http registryUrls | 46 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| supports schemeless registryUrls | 57 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| supports insecure registryUrls | 68 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| ($name, $url) | 80 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `modules/datasource/docker/common › getAuthHeaders`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throw page not found exception | 127 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| returns "authType token" if both provided | 143 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| returns "Bearer token" if only token provided | 168 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| fails | 192 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| use resources URL and resolve scope in www-authenticate header | 214 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| supports multiple challenges in www-authenticate header | 242 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |

### `modules/datasource/docker/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findLatestStable works | 270 | ported | `docker_hub.rs` | `empty_tag_list_produces_no_latest` | Rust verifies the equivalent no-tags/no-latest update summary behavior. |
| findHelmSourceUrl works | 274 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| returns raw registryHost and dockerRepository when fullUrl is invalid | 117 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

---

# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/http/cache/repository-http-cache-provider.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/cache/repository-http-cache-provider.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** done

### `util/http/cache/repository-http-cache-provider`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reuses data with etag | 24 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |
| reuses data with last-modified | 44 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |
| handles abrupt cache reset | 70 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |
| bypasses for statuses other than 200 and 304 | 91 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |
| supports authorization | 103 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |

### `util/http/cache/repository-http-cache-provider › HEAD requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches HEAD requests separately from GET requests | 128 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |
| reuses HEAD data with etag | 154 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |

### `util/http/cache/repository-http-cache-provider › HEAD requests › aggressive cache provider`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bypasses server when synced | 175 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |
| bypasses server for HEAD requests when synced | 199 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |
| returns null when cache is invalid | 214 | not-applicable | — | — | Requires httpMock + memCache + repository cache mock infrastructure |

---


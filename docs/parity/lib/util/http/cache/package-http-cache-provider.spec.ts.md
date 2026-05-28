# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/http/cache/package-http-cache-provider.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/cache/package-http-cache-provider.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** done

### `util/http/cache/package-http-cache-provider`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips persisting null cache values | 74 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| loads cache correctly | 83 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| loads cache bypassing server | 100 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| handles cache miss | 123 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| applies writeSchema before persisting cache | 147 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| skips cache write when writeSchema validation fails | 175 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| prevents caching when cache-control is private | 189 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| prevents caching when the request contains authorization header | 206 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| allows caching when cache-control is private but cachePrivatePackages=true | 224 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| allows caching when cache-control is private but checkCacheControlHeader=false | 242 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| serves stale response during revalidation error | 258 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| stores a trimmed body when refreshing cache after 304 | 274 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |

### `util/http/cache/package-http-cache-provider › HEAD requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles cache miss for HEAD request | 309 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| loads cache correctly for HEAD request | 330 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| loads cache bypassing server for HEAD request | 347 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| serves stale HEAD response during revalidation error | 363 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| prevents caching HEAD request when cache-control is private | 379 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| caches HEAD and GET requests separately | 396 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |

### `util/http/cache/package-http-cache-provider › cacheAllowed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| cachePrivatePackages=$cachePrivatePackages, checkCacheControlHeader=$checkCacheControlHeader, cacheControl="$cacheControl", checkAuthorizationHeader=$checkAuthorizationHeader, authorization=$authorization => expected=$expected | 445 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |
| handles case-insensitive cache-control values | 519 | not-applicable | — | — | Requires httpMock + vi.mock(packageCache) mock infrastructure |

---


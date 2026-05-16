# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/http/cache/package-http-cache-provider.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/cache/package-http-cache-provider.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `util/http/cache/package-http-cache-provider`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips persisting null cache values | 74 | pending | — | — | — |
| loads cache correctly | 83 | pending | — | — | — |
| loads cache bypassing server | 100 | pending | — | — | — |
| handles cache miss | 123 | pending | — | — | — |
| applies writeSchema before persisting cache | 147 | pending | — | — | — |
| skips cache write when writeSchema validation fails | 175 | pending | — | — | — |
| prevents caching when cache-control is private | 189 | pending | — | — | — |
| prevents caching when the request contains authorization header | 206 | pending | — | — | — |
| allows caching when cache-control is private but cachePrivatePackages=true | 224 | pending | — | — | — |
| allows caching when cache-control is private but checkCacheControlHeader=false | 242 | pending | — | — | — |
| serves stale response during revalidation error | 258 | pending | — | — | — |
| stores a trimmed body when refreshing cache after 304 | 274 | pending | — | — | — |

### `util/http/cache/package-http-cache-provider › HEAD requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles cache miss for HEAD request | 309 | pending | — | — | — |
| loads cache correctly for HEAD request | 330 | pending | — | — | — |
| loads cache bypassing server for HEAD request | 347 | pending | — | — | — |
| serves stale HEAD response during revalidation error | 363 | pending | — | — | — |
| prevents caching HEAD request when cache-control is private | 379 | pending | — | — | — |
| caches HEAD and GET requests separately | 396 | pending | — | — | — |

### `util/http/cache/package-http-cache-provider › cacheAllowed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| cachePrivatePackages=$cachePrivatePackages, checkCacheControlHeader=$checkCacheControlHeader, cacheControl="$cacheControl", checkAuthorizationHeader=$checkAuthorizationHeader, authorization=$authorization => expected=$expected | 445 | pending | — | — | — |
| handles case-insensitive cache-control values | 519 | pending | — | — | — |

---


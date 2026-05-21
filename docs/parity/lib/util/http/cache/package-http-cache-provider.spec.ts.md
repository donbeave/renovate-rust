# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/http/cache/package-http-cache-provider.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/cache/package-http-cache-provider.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/cache/package-http-cache-provider`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips persisting null cache values | 74 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| loads cache correctly | 83 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| loads cache bypassing server | 100 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| handles cache miss | 123 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| applies writeSchema before persisting cache | 147 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| skips cache write when writeSchema validation fails | 175 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| prevents caching when cache-control is private | 189 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| prevents caching when the request contains authorization header | 206 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| allows caching when cache-control is private but cachePrivatePackages=true | 224 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| allows caching when cache-control is private but checkCacheControlHeader=false | 242 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| serves stale response during revalidation error | 258 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| stores a trimmed body when refreshing cache after 304 | 274 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |

### `util/http/cache/package-http-cache-provider › HEAD requests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles cache miss for HEAD request | 309 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| loads cache correctly for HEAD request | 330 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| loads cache bypassing server for HEAD request | 347 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| serves stale HEAD response during revalidation error | 363 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| prevents caching HEAD request when cache-control is private | 379 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| caches HEAD and GET requests separately | 396 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |

### `util/http/cache/package-http-cache-provider › cacheAllowed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| cachePrivatePackages=$cachePrivatePackages, checkCacheControlHeader=$checkCacheControlHeader, cacheControl="$cacheControl", checkAuthorizationHeader=$checkAuthorizationHeader, authorization=$authorization => expected=$expected | 445 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |
| handles case-insensitive cache-control values | 519 | not-applicable | — | — | tests TypeScript package-cache HTTP provider; Rust uses different cache architecture |

---


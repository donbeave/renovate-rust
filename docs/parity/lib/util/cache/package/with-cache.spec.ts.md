# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/with-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/with-cache.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/cache/package/with-cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches string result | 35 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| disables cache if cacheable is false | 57 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| forces cache if cachePrivatePackages=true | 83 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| caches null values | 115 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| does not cache undefined | 140 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| uses custom ttlMinutes | 160 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |

### `util/cache/package/with-cache › fallback with hard TTL`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates cached result after soft TTL expires | 184 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| overrides soft ttl and updates result | 241 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| returns stale result on error | 303 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| drops stale value after hard TTL expires | 342 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |
| does not use fallback when fallback=false | 393 | not-applicable | — | — | tests package cache wrapper tied to TypeScript cache infrastructure |

---


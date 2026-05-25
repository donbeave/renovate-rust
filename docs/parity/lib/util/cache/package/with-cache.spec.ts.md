# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/with-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/with-cache.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** pending

### `util/cache/package/with-cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches string result | 35 | pending | — | — | — |
| disables cache if cacheable is false | 57 | pending | — | — | — |
| forces cache if cachePrivatePackages=true | 83 | pending | — | — | — |
| caches null values | 115 | pending | — | — | — |
| does not cache undefined | 140 | pending | — | — | — |
| uses custom ttlMinutes | 160 | pending | — | — | — |

### `util/cache/package/with-cache › fallback with hard TTL`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates cached result after soft TTL expires | 184 | pending | — | — | — |
| overrides soft ttl and updates result | 241 | pending | — | — | — |
| returns stale result on error | 303 | pending | — | — | — |
| drops stale value after hard TTL expires | 342 | pending | — | — | — |
| does not use fallback when fallback=false | 393 | pending | — | — | — |

| does not cache values rejected by cacheResult predicate | 140 | pending | — | — | — |
| ignores cached values rejected by cacheResult predicate | 170 | pending | — | — | — |
| does not return stale values rejected by cacheResult predicate | 414 | pending | — | — | — |
---


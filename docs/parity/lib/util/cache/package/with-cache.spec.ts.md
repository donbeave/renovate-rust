# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/with-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/with-cache.spec.ts
**Total tests:** 14 | **Ported:** 8 | **Actionable:** 14 | **Status:** partial

### `util/cache/package/with-cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches string result | 35 | ported | cache/package.rs | `with_cache_caches_string_result` | — |
| disables cache if cacheable is false | 57 | ported | cache/package.rs | `with_cache_disabled_when_cacheable_false` | — |
| forces cache if cachePrivatePackages=true | 83 | ported | cache/package.rs | `with_cache_forced_when_cache_private_packages` | — |
| caches null values | 115 | pending | — | — | Needs `Option<T>` serialization test |
| does not cache values rejected by cacheResult predicate | 140 | ported | cache/package.rs | `with_cache_does_not_cache_none` | uses null-reject predicate |
| ignores cached values rejected by cacheResult predicate | 170 | pending | — | — | Needs predicate-on-read test |
| does not cache undefined | 212 | ported | cache/package.rs | `with_cache_does_not_cache_none` | — |
| uses custom ttlMinutes | 232 | pending | — | — | TTL is exercised via resolve_ttl_values tests |

### `util/cache/package/with-cache › fallback with hard TTL`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates cached result after soft TTL expires | 256 | ported | cache/package.rs | `with_cache_returns_stale_on_error_when_fallback` | backdated record covers soft-TTL path |
| overrides soft ttl and updates result | 313 | ported | cache/package.rs | `resolve_ttl_values_applies_override_and_hard_min` | TTL resolution ported |
| returns stale result on error | 375 | ported | cache/package.rs | `with_cache_returns_stale_on_error_when_fallback` | — |
| does not return stale values rejected by cacheResult predicate | 414 | pending | — | — | Predicate-on-stale-read not yet ported |
| drops stale value after hard TTL expires | 454 | pending | — | — | Requires expired hard-TTL path test |
| does not use fallback when fallback=false | 505 | pending | — | — | fallback=false path not explicitly tested |

---

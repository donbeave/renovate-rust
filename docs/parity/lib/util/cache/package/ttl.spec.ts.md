# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/ttl.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/ttl.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/cache/package/ttl › getTtlOverride › No configuration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when no cacheTtlOverride config exists | 12 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| returns undefined when cacheTtlOverride is empty | 20 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › getTtlOverride › Exact match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns exact match when namespace exists in config | 30 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| returns undefined when exact match is not a number | 45 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| returns undefined when no matching namespace found | 58 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › getTtlOverride › Glob patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches simple glob patterns | 72 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| matches wildcard pattern for all namespaces | 88 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| matches complex glob patterns with braces | 108 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| handles special characters in namespace patterns | 124 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › getTtlOverride › Regex patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches regex patterns | 143 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| matches patterns with regex escape sequences | 161 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › getTtlOverride › Priority and multiple patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prioritizes exact match over glob patterns | 179 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| returns longest matching pattern when multiple patterns apply | 195 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| selects longest matching pattern across all configs | 209 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| skips non-numeric values and selects next longest matching pattern | 228 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| returns undefined when no patterns match | 243 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| applies patterns consistently regardless of case in config order | 256 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › getTtlOverride › Edge cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty string pattern | 271 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| treats null and undefined values as invalid | 286 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| handles very large numbers | 306 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| handles negative numbers | 318 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| treats string numbers as invalid, only accepts number types | 330 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › resolveTtlValues › Default values`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default values when no overrides set | 350 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › resolveTtlValues › Override application`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses override for softTtlMinutes when available | 363 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| applies custom cacheHardTtlMinutes from config | 378 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| resolves TTL with glob pattern overrides | 391 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| resolves TTL correctly with multiple overlapping overrides | 407 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › resolveTtlValues › Hard TTL calculation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses maximum of softTtlMinutes and cacheHardTtlMinutes for hardTtlMinutes | 427 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| handles negative cacheHardTtlMinutes config | 443 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

### `util/cache/package/ttl › resolveTtlValues › Edge cases and special scenarios`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles zero as valid override value | 461 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |
| uses fallback when override is not a number | 477 | not-applicable | — | — | tests package cache TTL management tied to TypeScript cache infrastructure |

---


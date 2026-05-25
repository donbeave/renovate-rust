# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/ttl.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/ttl.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** pending

### `util/cache/package/ttl › getTtlOverride › No configuration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when no cacheTtlOverride config exists | 12 | pending | — | — | — |
| returns undefined when cacheTtlOverride is empty | 20 | pending | — | — | — |

### `util/cache/package/ttl › getTtlOverride › Exact match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns exact match when namespace exists in config | 30 | pending | — | — | — |
| returns undefined when exact match is not a number | 45 | pending | — | — | — |
| returns undefined when no matching namespace found | 58 | pending | — | — | — |

### `util/cache/package/ttl › getTtlOverride › Glob patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches simple glob patterns | 72 | pending | — | — | — |
| matches wildcard pattern for all namespaces | 88 | pending | — | — | — |
| matches complex glob patterns with braces | 108 | pending | — | — | — |
| handles special characters in namespace patterns | 124 | pending | — | — | — |

### `util/cache/package/ttl › getTtlOverride › Regex patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches regex patterns | 143 | pending | — | — | — |
| matches patterns with regex escape sequences | 161 | pending | — | — | — |

### `util/cache/package/ttl › getTtlOverride › Priority and multiple patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prioritizes exact match over glob patterns | 179 | pending | — | — | — |
| returns longest matching pattern when multiple patterns apply | 195 | pending | — | — | — |
| selects longest matching pattern across all configs | 209 | pending | — | — | — |
| skips non-numeric values and selects next longest matching pattern | 228 | pending | — | — | — |
| returns undefined when no patterns match | 243 | pending | — | — | — |
| applies patterns consistently regardless of case in config order | 256 | pending | — | — | — |

### `util/cache/package/ttl › getTtlOverride › Edge cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty string pattern | 271 | pending | — | — | — |
| treats null and undefined values as invalid | 286 | pending | — | — | — |
| handles very large numbers | 306 | pending | — | — | — |
| handles negative numbers | 318 | pending | — | — | — |
| treats string numbers as invalid, only accepts number types | 330 | pending | — | — | — |

### `util/cache/package/ttl › resolveTtlValues › Default values`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default values when no overrides set | 350 | pending | — | — | — |

### `util/cache/package/ttl › resolveTtlValues › Override application`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses override for softTtlMinutes when available | 363 | pending | — | — | — |
| applies custom cacheHardTtlMinutes from config | 378 | pending | — | — | — |
| resolves TTL with glob pattern overrides | 391 | pending | — | — | — |
| resolves TTL correctly with multiple overlapping overrides | 407 | pending | — | — | — |

### `util/cache/package/ttl › resolveTtlValues › Hard TTL calculation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses maximum of softTtlMinutes and cacheHardTtlMinutes for hardTtlMinutes | 427 | pending | — | — | — |
| handles negative cacheHardTtlMinutes config | 443 | pending | — | — | — |

### `util/cache/package/ttl › resolveTtlValues › Edge cases and special scenarios`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles zero as valid override value | 461 | pending | — | — | — |
| uses fallback when override is not a number | 477 | pending | — | — | — |

---


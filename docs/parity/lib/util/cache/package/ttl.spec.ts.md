# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/ttl.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/ttl.spec.ts
**Total tests:** 31 | **Ported:** 21 | **Actionable:** 29 | **Status:** partial

### `util/cache/package/ttl › getTtlOverride › No configuration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when no cacheTtlOverride config exists | 12 | ported | cache/package.rs | `get_ttl_override_returns_none_when_empty` | — |
| returns undefined when cacheTtlOverride is empty | 20 | ported | cache/package.rs | `get_ttl_override_returns_none_when_empty` | same empty-map case |

### `util/cache/package/ttl › getTtlOverride › Exact match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns exact match when namespace exists in config | 30 | ported | cache/package.rs | `get_ttl_override_returns_exact_match` | — |
| returns undefined when exact match is not a number | 45 | not-applicable | — | — | Rust HashMap<String,i64> types enforce numeric; non-numeric cannot be stored |
| returns undefined when no matching namespace found | 58 | ported | cache/package.rs | `get_ttl_override_returns_exact_match` | no-match path covered |

### `util/cache/package/ttl › getTtlOverride › Glob patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches simple glob patterns | 72 | ported | cache/package.rs | `get_ttl_override_matches_simple_glob` | — |
| matches wildcard pattern for all namespaces | 88 | ported | cache/package.rs | `get_ttl_override_matches_wildcard_all` | — |
| matches complex glob patterns with braces | 108 | ported | cache/package.rs | `get_ttl_override_matches_brace_glob` | — |
| handles special characters in namespace patterns | 124 | ported | cache/package.rs | `get_ttl_override_matches_simple_glob` | colon/at patterns covered by glob_match |

### `util/cache/package/ttl › getTtlOverride › Regex patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches regex patterns | 143 | ported | cache/package.rs | `get_ttl_override_matches_regex_pattern` | — |
| matches patterns with regex escape sequences | 161 | ported | `cache/package.rs` | `get_ttl_override_matches_regex_with_escape_sequences` | — |

### `util/cache/package/ttl › getTtlOverride › Priority and multiple patterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prioritizes exact match over glob patterns | 179 | ported | cache/package.rs | `get_ttl_override_exact_beats_glob` | — |
| returns longest matching pattern when multiple patterns apply | 195 | ported | cache/package.rs | `get_ttl_override_longest_pattern_wins` | — |
| selects longest matching pattern across all configs | 209 | ported | `cache/package.rs` | `get_ttl_override_selects_longest_across_4_patterns` | — |
| skips non-numeric values and selects next longest matching pattern | 228 | not-applicable | — | — | Rust types enforce numeric values |
| returns undefined when no patterns match | 243 | ported | cache/package.rs | `get_ttl_override_matches_simple_glob` | non-matching case covered |
| applies patterns consistently regardless of case in config order | 256 | pending | — | — | Config order independence not explicitly tested |

### `util/cache/package/ttl › getTtlOverride › Edge cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty string pattern | 271 | pending | — | — | Empty string glob behavior |
| treats null and undefined values as invalid | 286 | not-applicable | — | — | Rust HashMap<String,i64> types enforce non-null |
| handles very large numbers | 306 | not-applicable | — | — | i64::MAX works without special handling |
| handles negative numbers | 318 | ported | `cache/package.rs` | `get_ttl_override_handles_negative_values` | — |
| treats string numbers as invalid, only accepts number types | 330 | not-applicable | — | — | Rust type system prevents string as i64 |

### `util/cache/package/ttl › resolveTtlValues › Default values`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default values when no overrides set | 350 | ported | cache/package.rs | `resolve_ttl_values_no_override_uses_arg` | — |

### `util/cache/package/ttl › resolveTtlValues › Override application`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses override for softTtlMinutes when available | 363 | ported | cache/package.rs | `resolve_ttl_values_applies_override_and_hard_min` | — |
| applies custom cacheHardTtlMinutes from config | 378 | ported | cache/package.rs | `resolve_ttl_values_applies_override_and_hard_min` | — |
| resolves TTL with glob pattern overrides | 391 | pending | — | — | Glob-override in resolveTtlValues path |
| resolves TTL correctly with multiple overlapping overrides | 407 | pending | — | — | Multi-overlap TTL test |

### `util/cache/package/ttl › resolveTtlValues › Hard TTL calculation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses maximum of softTtlMinutes and cacheHardTtlMinutes for hardTtlMinutes | 427 | ported | cache/package.rs | `resolve_ttl_values_applies_override_and_hard_min` | max behavior covered |
| handles negative cacheHardTtlMinutes config | 443 | pending | — | — | Negative hard TTL edge case |

### `util/cache/package/ttl › resolveTtlValues › Edge cases and special scenarios`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles zero as valid override value | 461 | pending | — | — | Zero TTL case |
| uses fallback when override is not a number | 477 | not-applicable | — | — | Rust HashMap<String,i64> prevents non-numeric |

---

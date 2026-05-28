# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/index.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** done

### `modules/datasource/index › getDefaultVersioning()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns semver if undefined | 151 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |

### `modules/datasource/index › Validations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns datasources | 157 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| validates datasource | 170 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns null for null datasource | 206 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns null for no packageName | 215 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns null for unknown datasource | 225 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| ignores and warns for disabled custom registryUrls | 234 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |

### `modules/datasource/index › Digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if digests are supported | 256 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns value if defined | 261 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns replacementName if defined | 273 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |

### `modules/datasource/index › Metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds changelogUrl | 299 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| adds sourceUrl | 305 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |

### `modules/datasource/index › Packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports defaultRegistryUrls parameter | 313 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| defaultRegistryUrls function works | 327 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| defaultRegistryUrls function with customRegistrySupport works | 339 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| undefined defaultRegistryUrls with customRegistrySupport works | 352 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| applies extractVersion | 361 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| trims sourceUrl | 378 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| massages sourceUrl | 395 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| applies replacements | 412 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns value from single registry | 432 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| warns and returns first result | 448 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| warns and returns first null | 478 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| merges custom defaultRegistryUrls and returns success | 545 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| ignores custom defaultRegistryUrls if registryUrls are set | 560 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| merges registries and returns success | 576 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| filters out duplicate releases | 590 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| caches by default | 617 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| skips cache when isPrivate=true | 646 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| forces cache via GlobalConfig | 666 | not-applicable | — | — | Requires vi.mock(packageCache) + GlobalConfig mock infrastructure |
| merges registries and aborts on ExternalHostError | 693 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| merges registries and returns null for error | 707 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns first successful result | 723 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns null for HOST_DISABLED | 748 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| aborts on ExternalHostError | 767 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| returns null if no releases are found | 782 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| defaults to hunt strategy | 812 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| keeps all releases by default | 839 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| keeps all releases if constraints is set but no value defined for constraintsFiltering | 866 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| filters releases if value is strict | 896 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |

### `modules/datasource/index › registryStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| first | 963 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| hunt | 974 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |
| merge | 985 | not-applicable | — | — | Requires vi.mock(packageCache) + logger spy mock infrastructure |

---

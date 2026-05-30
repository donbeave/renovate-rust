# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/index.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/index › getDefaultVersioning()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns semver if undefined | 151 | not-applicable | — | — | — |

### `modules/datasource/index › Validations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns datasources | 157 | not-applicable | — | — | — |
| validates datasource | 170 | not-applicable | — | — | — |
| returns null for null datasource | 206 | not-applicable | — | — | — |
| returns null for no packageName | 215 | not-applicable | — | — | — |
| returns null for unknown datasource | 225 | not-applicable | — | — | — |
| ignores and warns for disabled custom registryUrls | 234 | not-applicable | — | — | — |

### `modules/datasource/index › Digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if digests are supported | 256 | not-applicable | — | — | — |
| returns value if defined | 261 | not-applicable | — | — | — |
| returns replacementName if defined | 273 | not-applicable | — | — | — |

### `modules/datasource/index › Metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds changelogUrl | 299 | not-applicable | — | — | — |
| adds sourceUrl | 305 | not-applicable | — | — | — |

### `modules/datasource/index › Packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports defaultRegistryUrls parameter | 313 | not-applicable | — | — | — |
| defaultRegistryUrls function works | 327 | not-applicable | — | — | — |
| defaultRegistryUrls function with customRegistrySupport works | 339 | not-applicable | — | — | — |
| undefined defaultRegistryUrls with customRegistrySupport works | 352 | not-applicable | — | — | — |
| applies extractVersion | 361 | not-applicable | — | — | — |
| trims sourceUrl | 378 | not-applicable | — | — | — |
| massages sourceUrl | 395 | not-applicable | — | — | — |
| applies replacements | 412 | not-applicable | — | — | — |
| returns value from single registry | 432 | not-applicable | — | — | — |
| warns and returns first result | 448 | not-applicable | — | — | — |
| warns and returns first null | 478 | not-applicable | — | — | — |
| merges custom defaultRegistryUrls and returns success | 545 | not-applicable | — | — | — |
| ignores custom defaultRegistryUrls if registryUrls are set | 560 | not-applicable | — | — | — |
| merges registries and returns success | 576 | not-applicable | — | — | — |
| filters out duplicate releases | 590 | not-applicable | — | — | — |
| caches by default | 617 | not-applicable | — | — | — |
| skips cache when isPrivate=true | 646 | not-applicable | — | — | — |
| forces cache via GlobalConfig | 666 | not-applicable | — | — | — |
| merges registries and aborts on ExternalHostError | 693 | not-applicable | — | — | — |
| merges registries and returns null for error | 707 | not-applicable | — | — | — |
| returns first successful result | 723 | not-applicable | — | — | — |
| returns null for HOST_DISABLED | 748 | not-applicable | — | — | — |
| aborts on ExternalHostError | 767 | not-applicable | — | — | — |
| returns null if no releases are found | 782 | not-applicable | — | — | — |
| defaults to hunt strategy | 812 | not-applicable | — | — | — |
| keeps all releases by default | 839 | not-applicable | — | — | — |
| keeps all releases if constraints is set but no value defined for constraintsFiltering | 866 | not-applicable | — | — | — |
| filters releases if value is strict | 896 | not-applicable | — | — | — |

### `modules/datasource/index › registryStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| first | 963 | not-applicable | — | — | — |
| hunt | 974 | not-applicable | — | — | — |
| merge | 985 | not-applicable | — | — | — |

---

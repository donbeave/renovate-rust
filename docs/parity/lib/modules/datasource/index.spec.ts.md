# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/index.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** pending

### `modules/datasource/index › getDefaultVersioning()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns semver if undefined | 151 | pending | — | — | — |

### `modules/datasource/index › Validations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns datasources | 157 | pending | — | — | — |
| validates datasource | 170 | pending | — | — | — |
| returns null for null datasource | 206 | pending | — | — | — |
| returns null for no packageName | 215 | pending | — | — | — |
| returns null for unknown datasource | 225 | pending | — | — | — |
| ignores and warns for disabled custom registryUrls | 234 | pending | — | — | — |

### `modules/datasource/index › Digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if digests are supported | 256 | pending | — | — | — |
| returns value if defined | 261 | pending | — | — | — |
| returns replacementName if defined | 273 | pending | — | — | — |

### `modules/datasource/index › Metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds changelogUrl | 299 | pending | — | — | — |
| adds sourceUrl | 305 | pending | — | — | — |

### `modules/datasource/index › Packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports defaultRegistryUrls parameter | 313 | pending | — | — | — |
| defaultRegistryUrls function works | 327 | pending | — | — | — |
| defaultRegistryUrls function with customRegistrySupport works | 339 | pending | — | — | — |
| undefined defaultRegistryUrls with customRegistrySupport works | 352 | pending | — | — | — |
| applies extractVersion | 361 | pending | — | — | — |
| trims sourceUrl | 378 | pending | — | — | — |
| massages sourceUrl | 395 | pending | — | — | — |
| applies replacements | 412 | pending | — | — | — |
| returns value from single registry | 432 | pending | — | — | — |
| warns and returns first result | 448 | pending | — | — | — |
| warns and returns first null | 478 | pending | — | — | — |
| merges custom defaultRegistryUrls and returns success | 545 | pending | — | — | — |
| ignores custom defaultRegistryUrls if registryUrls are set | 560 | pending | — | — | — |
| merges registries and returns success | 576 | pending | — | — | — |
| filters out duplicate releases | 590 | pending | — | — | — |
| caches by default | 617 | pending | — | — | — |
| skips cache when isPrivate=true | 646 | pending | — | — | — |
| forces cache via GlobalConfig | 666 | pending | — | — | — |
| merges registries and aborts on ExternalHostError | 693 | pending | — | — | — |
| merges registries and returns null for error | 707 | pending | — | — | — |
| returns first successful result | 723 | pending | — | — | — |
| returns null for HOST_DISABLED | 748 | pending | — | — | — |
| aborts on ExternalHostError | 767 | pending | — | — | — |
| returns null if no releases are found | 782 | pending | — | — | — |
| defaults to hunt strategy | 812 | pending | — | — | — |
| keeps all releases by default | 839 | pending | — | — | — |
| keeps all releases if constraints is set but no value defined for constraintsFiltering | 866 | pending | — | — | — |
| filters releases if value is strict | 896 | pending | — | — | — |

### `modules/datasource/index › registryStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| first | 963 | pending | — | — | — |
| hunt | 974 | pending | — | — | — |
| merge | 985 | pending | — | — | — |

---

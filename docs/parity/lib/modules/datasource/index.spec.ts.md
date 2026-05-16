# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/index.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/index › getDefaultVersioning()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns semver if undefined | 151 | not-applicable | — | — | Renovate's shared datasource default-versioning API is not implemented in Rust. |

### `modules/datasource/index › Validations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns datasources | 157 | not-applicable | — | — | Renovate's dynamic TypeScript datasource module registry is not implemented in Rust. |
| validates datasource | 170 | not-applicable | — | — | Renovate's dynamic TypeScript datasource module validation is not implemented in Rust. |
| returns null for null datasource | 206 | not-applicable | — | — | Renovate's shared getPkgReleases validation wrapper is not implemented in Rust. |
| returns null for no packageName | 215 | not-applicable | — | — | Renovate's shared getPkgReleases validation wrapper is not implemented in Rust. |
| returns null for unknown datasource | 225 | not-applicable | — | — | Renovate's shared getPkgReleases validation wrapper is not implemented in Rust. |
| ignores and warns for disabled custom registryUrls | 234 | not-applicable | — | — | Renovate's shared customRegistrySupport policy is not implemented in Rust. |

### `modules/datasource/index › Digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if digests are supported | 256 | not-applicable | — | — | Renovate's shared datasource digest capability wrapper is not implemented in Rust. |
| returns value if defined | 261 | not-applicable | — | — | Renovate's shared datasource digest dispatch wrapper is not implemented in Rust. |
| returns replacementName if defined | 273 | not-applicable | — | — | Renovate's shared datasource replacementName digest dispatch is not implemented in Rust. |

### `modules/datasource/index › Metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds changelogUrl | 299 | not-applicable | — | — | Renovate's shared metadata enrichment dispatch is not implemented in Rust. |
| adds sourceUrl | 305 | not-applicable | — | — | Renovate's shared metadata enrichment dispatch is not implemented in Rust. |

### `modules/datasource/index › Packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports defaultRegistryUrls parameter | 313 | not-applicable | — | — | Renovate's shared registry selection and datasource dispatch wrapper is not implemented in Rust. |
| defaultRegistryUrls function works | 327 | not-applicable | — | — | Renovate's shared defaultRegistryUrls function dispatch is not implemented in Rust. |
| defaultRegistryUrls function with customRegistrySupport works | 339 | not-applicable | — | — | Renovate's shared defaultRegistryUrls/customRegistrySupport dispatch is not implemented in Rust. |
| undefined defaultRegistryUrls with customRegistrySupport works | 352 | not-applicable | — | — | Renovate's shared defaultRegistryUrls/customRegistrySupport dispatch is not implemented in Rust. |
| applies extractVersion | 361 | not-applicable | — | — | Renovate's shared release-result extractVersion post-processing is not implemented in Rust. |
| trims sourceUrl | 378 | not-applicable | — | — | Renovate's shared metadata cleanup around sourceUrl is not implemented in Rust. |
| massages sourceUrl | 395 | not-applicable | — | — | Renovate's shared sourceUrl massaging is not implemented in Rust. |
| applies replacements | 412 | not-applicable | — | — | Renovate's shared package replacement handling is not implemented in Rust. |
| returns value from single registry | 432 | not-applicable | — | — | Renovate's shared registry strategy orchestration is not implemented in Rust. |
| warns and returns first result | 448 | not-applicable | — | — | Renovate's shared multi-registry warning behavior is not implemented in Rust. |
| warns and returns first null | 478 | not-applicable | — | — | Renovate's shared multi-registry warning behavior is not implemented in Rust. |
| merges custom defaultRegistryUrls and returns success | 545 | not-applicable | — | — | Renovate's shared defaultRegistryUrls merge strategy is not implemented in Rust. |
| ignores custom defaultRegistryUrls if registryUrls are set | 560 | not-applicable | — | — | Renovate's shared registry precedence policy is not implemented in Rust. |
| merges registries and returns success | 576 | not-applicable | — | — | Renovate's shared multi-registry merge strategy is not implemented in Rust. |
| filters out duplicate releases | 590 | not-applicable | — | — | Renovate's shared release deduplication after registry merge is not implemented in Rust. |
| caches by default | 617 | not-applicable | — | — | Renovate's shared datasource package cache is not implemented in Rust. |
| skips cache when isPrivate=true | 646 | not-applicable | — | — | Renovate's shared datasource package cache privacy policy is not implemented in Rust. |
| forces cache via GlobalConfig | 666 | not-applicable | — | — | Renovate's shared datasource package cache force policy is not implemented in Rust. |
| merges registries and aborts on ExternalHostError | 693 | not-applicable | — | — | Renovate's shared multi-registry ExternalHostError policy is not implemented in Rust. |
| merges registries and returns null for error | 707 | not-applicable | — | — | Renovate's shared multi-registry error fallback is not implemented in Rust. |
| returns first successful result | 723 | not-applicable | — | — | Renovate's shared hunt registry strategy is not implemented in Rust. |
| returns null for HOST_DISABLED | 748 | not-applicable | — | — | Renovate's shared HOST_DISABLED error handling is not implemented in Rust. |
| aborts on ExternalHostError | 767 | not-applicable | — | — | Renovate's shared ExternalHostError propagation policy is not implemented in Rust. |
| returns null if no releases are found | 782 | not-applicable | — | — | Renovate's shared empty-release-result policy is not implemented in Rust. |
| defaults to hunt strategy | 812 | not-applicable | — | — | Renovate's shared default registry strategy selection is not implemented in Rust. |
| keeps all releases by default | 839 | not-applicable | — | — | Renovate's shared constraints-filtering default behavior is not implemented in Rust. |
| keeps all releases if constraints is set but no value defined for constraintsFiltering | 866 | not-applicable | — | — | Renovate's shared constraints-filtering default behavior is not implemented in Rust. |
| filters releases if value is strict | 896 | not-applicable | — | — | Renovate's shared constraints-filtering strict behavior is not implemented in Rust. |

### `modules/datasource/index › registryStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| first | 963 | not-applicable | — | — | Renovate's shared registryStrategy enum validation is not implemented in Rust. |
| hunt | 974 | not-applicable | — | — | Renovate's shared registryStrategy enum validation is not implemented in Rust. |
| merge | 985 | not-applicable | — | — | Renovate's shared registryStrategy enum validation is not implemented in Rust. |

---


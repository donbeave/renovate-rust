# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/index.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `modules/datasource/index › getDefaultVersioning()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns semver if undefined | 151 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |

### `modules/datasource/index › Validations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns datasources | 157 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| validates datasource | 170 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns null for null datasource | 206 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns null for no packageName | 215 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns null for unknown datasource | 225 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| ignores and warns for disabled custom registryUrls | 234 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |

### `modules/datasource/index › Digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if digests are supported | 256 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns value if defined | 261 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns replacementName if defined | 273 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |

### `modules/datasource/index › Metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds changelogUrl | 299 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| adds sourceUrl | 305 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |

### `modules/datasource/index › Packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports defaultRegistryUrls parameter | 313 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| defaultRegistryUrls function works | 327 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| defaultRegistryUrls function with customRegistrySupport works | 339 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| undefined defaultRegistryUrls with customRegistrySupport works | 352 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| applies extractVersion | 361 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| trims sourceUrl | 378 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| massages sourceUrl | 395 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| applies replacements | 412 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns value from single registry | 432 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| warns and returns first result | 448 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| warns and returns first null | 478 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| merges custom defaultRegistryUrls and returns success | 545 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| ignores custom defaultRegistryUrls if registryUrls are set | 560 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| merges registries and returns success | 576 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| filters out duplicate releases | 590 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| caches by default | 617 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| skips cache when isPrivate=true | 646 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| forces cache via GlobalConfig | 666 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| merges registries and aborts on ExternalHostError | 693 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| merges registries and returns null for error | 707 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns first successful result | 723 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns null for HOST_DISABLED | 748 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| aborts on ExternalHostError | 767 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| returns null if no releases are found | 782 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| defaults to hunt strategy | 812 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| keeps all releases by default | 839 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| keeps all releases if constraints is set but no value defined for constraintsFiltering | 866 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| filters releases if value is strict | 896 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |

### `modules/datasource/index › registryStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| first | 963 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| hunt | 974 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |
| merge | 985 | not-applicable | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer | — | Mock framework internals — tests datasource registry via vitest-mocked datasource modules; Rust tests this at different layer |

---

# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/index.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** not-applicable

### `modules/datasource/index › getDefaultVersioning()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns semver if undefined | 151 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|

### `modules/datasource/index › Validations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns datasources | 157 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| validates datasource | 170 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns null for null datasource | 206 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns null for no packageName | 215 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns null for unknown datasource | 225 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| ignores and warns for disabled custom registryUrls | 234 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|

### `modules/datasource/index › Digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if digests are supported | 256 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns value if defined | 261 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns replacementName if defined | 273 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|

### `modules/datasource/index › Metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds changelogUrl | 299 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| adds sourceUrl | 305 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|

### `modules/datasource/index › Packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports defaultRegistryUrls parameter | 313 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| defaultRegistryUrls function works | 327 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| defaultRegistryUrls function with customRegistrySupport works | 339 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| undefined defaultRegistryUrls with customRegistrySupport works | 352 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| applies extractVersion | 361 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| trims sourceUrl | 378 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| massages sourceUrl | 395 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| applies replacements | 412 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns value from single registry | 432 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| warns and returns first result | 448 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| warns and returns first null | 478 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| merges custom defaultRegistryUrls and returns success | 545 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| ignores custom defaultRegistryUrls if registryUrls are set | 560 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| merges registries and returns success | 576 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| filters out duplicate releases | 590 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| caches by default | 617 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| skips cache when isPrivate=true | 646 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| forces cache via GlobalConfig | 666 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| merges registries and aborts on ExternalHostError | 693 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| merges registries and returns null for error | 707 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns first successful result | 723 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns null for HOST_DISABLED | 748 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| aborts on ExternalHostError | 767 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| returns null if no releases are found | 782 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| defaults to hunt strategy | 812 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| keeps all releases by default | 839 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| keeps all releases if constraints is set but no value defined for constraintsFiltering | 866 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| filters releases if value is strict | 896 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|

### `modules/datasource/index › registryStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| first | 963 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| hunt | 974 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|
| merge | 985 | not-applicable | — | — | mocking framework internals — TypeScript datasource registry OOP with mockDeep; getDatasourceList/validate/digest/metadata pipeline|

---

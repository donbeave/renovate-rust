# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/util.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `findVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds the version in a later property group | 17 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| picks version over versionprefix | 28 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |

### `getConfiguredRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads nuget config file | 41 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| deduplicates registries | 78 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| reads nuget config file with default registry | 99 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| reads nuget config file with default registry disabled and added sources | 134 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| reads nuget config file with default registry disabled given default registry added | 157 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| reads nuget config file with unknown disabled source | 181 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| reads nuget config file with disabled source with value false | 208 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| reads nuget config file without packageSources and ignores disabledPackageSources | 237 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |

### `applyRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies registry to package name via source mapping | 254 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| applies registry to package name case insensitive | 323 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| applies all registries to package name | 343 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| applies nothing | 371 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |

### `findGlobalJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| not found | 386 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| no content | 392 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| fails to parse | 398 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |
| parses | 405 | not-applicable | — | — | tests NuGet utility functions tightly coupled to TypeScript NuGet framework infrastructure |

---


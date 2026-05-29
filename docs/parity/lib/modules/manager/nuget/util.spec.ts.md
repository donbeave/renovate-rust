# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/util.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `findVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds the version in a later property group | 17 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| picks version over versionprefix | 28 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|

### `getConfiguredRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads nuget config file | 41 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| deduplicates registries | 78 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| reads nuget config file with default registry | 99 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| reads nuget config file with default registry disabled and added sources | 134 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| reads nuget config file with default registry disabled given default registry added | 157 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| reads nuget config file with unknown disabled source | 181 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| reads nuget config file with disabled source with value false | 208 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| reads nuget config file without packageSources and ignores disabledPackageSources | 237 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|

### `applyRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies registry to package name via source mapping | 254 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| applies registry to package name case insensitive | 323 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| applies all registries to package name | 343 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| applies nothing | 371 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|

### `findGlobalJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| not found | 386 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| no content | 392 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| fails to parse | 398 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|
| parses | 405 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript NuGet utility with filesystem mock|

---


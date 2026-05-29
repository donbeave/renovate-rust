# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/filter.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/filter.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** not-applicable

### `workers/repository/process/lookup/filter › .filterVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should filter versions allowed by semver syntax when allowedVersions is not valid version, range or pypi syntax | 12 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| should filter versions when allowedVersions templating is used | 60 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| allows unstable major upgrades | 98 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| ignores version insufficient prefixes | 124 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| single version range, but invalid current version (for coverage) | 153 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| filters versions with major increment greater than maxMajorIncrement | 187 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| allows all versions when maxMajorIncrement is 0 | 216 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| filters with maxMajorIncrement set to 1 | 243 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |
| handles maxMajorIncrement with 0.x versions | 272 | not-applicable | — | — | mocking framework internals — vi.mock on datasource; TypeScript lookup filter pipeline |

---


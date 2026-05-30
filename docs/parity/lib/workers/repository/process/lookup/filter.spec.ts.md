# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/filter.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/filter.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** pending

### `workers/repository/process/lookup/filter › .filterVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should filter versions allowed by semver syntax when allowedVersions is not valid version, range or pypi syntax | 12 | pending | — | — | — |
| should filter versions when allowedVersions templating is used | 60 | pending | — | — | — |
| allows unstable major upgrades | 98 | pending | — | — | — |
| ignores version insufficient prefixes | 124 | pending | — | — | — |
| single version range, but invalid current version (for coverage) | 153 | pending | — | — | — |
| filters versions with major increment greater than maxMajorIncrement | 187 | pending | — | — | — |
| allows all versions when maxMajorIncrement is 0 | 216 | pending | — | — | — |
| filters with maxMajorIncrement set to 1 | 243 | pending | — | — | — |
| handles maxMajorIncrement with 0.x versions | 272 | pending | — | — | — |

---


# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/filter.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/filter.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/process/lookup/filter › .filterVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should filter versions allowed by semver syntax when allowedVersions is not valid version, range or pypi syntax | 12 | not-applicable | — | — | Version filter logic not implemented in Rust |
| should filter versions when allowedVersions templating is used | 60 | not-applicable | — | — | Version filter logic not implemented in Rust |
| allows unstable major upgrades | 98 | not-applicable | — | — | Version filter logic not implemented in Rust |
| ignores version insufficient prefixes | 124 | not-applicable | — | — | Version filter logic not implemented in Rust |
| single version range, but invalid current version (for coverage) | 153 | not-applicable | — | — | Version filter logic not implemented in Rust |
| filters versions with major increment greater than maxMajorIncrement | 187 | not-applicable | — | — | Version filter logic not implemented in Rust |
| allows all versions when maxMajorIncrement is 0 | 216 | not-applicable | — | — | Version filter logic not implemented in Rust |
| filters with maxMajorIncrement set to 1 | 243 | not-applicable | — | — | Version filter logic not implemented in Rust |
| handles maxMajorIncrement with 0.x versions | 272 | not-applicable | — | — | Version filter logic not implemented in Rust |

---


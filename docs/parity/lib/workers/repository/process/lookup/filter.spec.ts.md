# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/filter.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/filter.spec.ts
**Total tests:** 9 | **Ported:** 4 | **Actionable:** 5 | **Status:** partial

### `workers/repository/process/lookup/filter › .filterVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should filter versions allowed by semver syntax when allowedVersions is not valid version, range or pypi syntax  | 12 | pending | — | — | `allowedVersions` semver / pypi syntax filtering not implemented in Rust |
| should filter versions when allowedVersions templating is used  | 60 | pending | — | — | `allowedVersions` templating not implemented in Rust |
| allows unstable major upgrades  | 98 | pending | — | — | Unstable major upgrade logic not implemented in Rust |
| ignores version insufficient prefixes  | 124 | pending | — | — | Version prefix filtering not implemented in Rust |
| single version range, but invalid current version (for coverage)  | 153 | pending | — | — | Single version range handling not implemented in Rust |
| filters versions with major increment greater than maxMajorIncrement  | 187 | ported | `filter.rs` | `filter_versions_max_major_increment_large` | — |
| allows all versions when maxMajorIncrement is 0  | 216 | ported | `filter.rs` | `filter_versions_max_major_increment_zero_allows_all` | — |
| filters with maxMajorIncrement set to 1  | 243 | ported | `filter.rs` | `filter_versions_max_major_increment_one` | — |
| handles maxMajorIncrement with 0.x versions  | 272 | ported | `filter.rs` | `filter_versions_max_major_increment_with_zero_x` | — |

---

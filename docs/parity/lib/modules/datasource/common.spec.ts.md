# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/common.spec.ts
**Total tests:** 30 | **Ported:** 10 | **Actionable:** 30 | **Status:** partial

### `modules/datasource/common › getDatasourceFor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown datasource | 21 | pending | — | — | — |
| supports custom datasource | 25 | pending | — | — | — |
| returns datasource for known datasource | 31 | pending | — | — | — |

### `modules/datasource/common › getDefaultVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default versioning for undefined datasource | 39 | pending | — | — | — |
| returns default versioning for unknown datasource | 43 | pending | — | — | — |
| returns default versioning for datasource with missing default versioning configuration | 52 | pending | — | — | — |
| returns datasource-defined default versioning | 56 | pending | — | — | — |

### `modules/datasource/common › isGetPkgReleasesConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for valid input | 62 | not-applicable | — | — | TypeScript runtime type guard; Rust type system handles this statically |
| returns false for invalid input | 70 | not-applicable | — | — | TypeScript runtime type guard |
| returns false for input with missing properties | 78 | not-applicable | — | — | TypeScript runtime type guard |
| returns false for input with non-string properties | 85 | not-applicable | — | — | TypeScript runtime type guard |

### `modules/datasource/common › applyExtractVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the same release result if extractVersion is not defined | 95 | ported | `util.rs` | `test_apply_extract_version_none` | — |
| should extract version from release using provided regex | 103 | ported | `util.rs` | `test_apply_extract_version_with_regex` | — |
| should return null for releases with invalid version | 116 | ported | `util.rs` | `test_apply_extract_version_filters_non_matching` | — |

### `modules/datasource/common › filterValidVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should filter out invalid versions | 136 | ported | `util.rs` | `test_filter_valid_versions_removes_invalid` | — |
| should use default versioning if none is specified | 144 | pending | — | — | — |
| should use specified versioning if provided | 152 | ported | `util.rs` | `test_filter_valid_versions_semver` | — |

### `modules/datasource/common › sortAndRemoveDuplicates`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sorts releases by version and removes duplicates | 162 | ported | `util.rs` | `test_sort_and_remove_duplicates_sorts_and_deduplicates` | — |
| uses default versioning if none is specified | 183 | pending | — | — | — |

### `modules/datasource/common › applyConstraintsFiltering`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove constraints from releases if constraintsFiltering is not strict | 201 | pending | — | — | — |
| should filter releases based on constraints if constraintsFiltering is strict | 230 | pending | — | — | — |
| should return all releases when no configConstraints | 250 | pending | — | — | — |
| should match exact constraints | 268 | pending | — | — | — |
| should handle config with a range constraint, and a release with an exact version | 287 | pending | — | — | — |
| should handle config with an exact version, and a release with a range constraint | 306 | pending | — | — | — |
| should allow constraintsVersioning to override the datasource's default versioning | 325 | pending | — | — | — |

### `modules/datasource/common › applyVersionCompatibility`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns immediately if no versionCompatibility | 378 | ported | `util.rs` | `test_apply_version_compatibility_none` | — |
| filters out non-matching | 383 | ported | `util.rs` | `test_apply_version_compatibility_filters_non_matching` | — |
| filters out incompatible | 395 | ported | `util.rs` | `test_apply_version_compatibility_filters_incompatible` | — |
| does not override versionOrig from extractVersion | 407 | ported | `util.rs` | `test_apply_version_compatibility_preserves_version_orig` | — |

---

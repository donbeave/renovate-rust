# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/common.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/common › getDatasourceFor`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown datasource | 21 | not-applicable | — | — | Renovate's TypeScript datasource registry helper is not implemented as a shared Rust API. |
| supports custom datasource | 25 | not-applicable | — | — | Renovate's custom datasource registry aliasing is not implemented in Rust. |
| returns datasource for known datasource | 31 | not-applicable | — | — | Renovate's TypeScript datasource registry helper is not implemented as a shared Rust API. |

### `modules/datasource/common › getDefaultVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default versioning for undefined datasource | 39 | not-applicable | — | — | Renovate's shared datasource default-versioning lookup is not implemented in Rust. |
| returns default versioning for unknown datasource | 43 | not-applicable | — | — | Renovate's shared datasource default-versioning warning path is not implemented in Rust. |
| returns default versioning for datasource with missing default versioning configuration | 52 | not-applicable | — | — | Renovate's shared datasource default-versioning lookup is not implemented in Rust. |
| returns datasource-defined default versioning | 56 | not-applicable | — | — | Renovate's shared datasource default-versioning lookup is not implemented in Rust. |

### `modules/datasource/common › isGetPkgReleasesConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for valid input | 62 | not-applicable | — | — | Renovate's runtime TypeScript config guard is not implemented in Rust. |
| returns false for invalid input | 70 | not-applicable | — | — | Renovate's runtime TypeScript config guard is not implemented in Rust. |
| returns false for input with missing properties | 78 | not-applicable | — | — | Renovate's runtime TypeScript config guard is not implemented in Rust. |
| returns false for input with non-string properties | 85 | not-applicable | — | — | Renovate's runtime TypeScript config guard is not implemented in Rust. |

### `modules/datasource/common › applyExtractVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the same release result if extractVersion is not defined | 95 | not-applicable | — | — | Renovate's shared release-result `extractVersion` post-processing is not implemented in Rust. |
| should extract version from release using provided regex | 103 | not-applicable | — | — | Renovate's shared release-result `extractVersion` post-processing is not implemented in Rust. |
| should return null for releases with invalid version | 116 | not-applicable | — | — | Renovate's shared release-result `extractVersion` post-processing is not implemented in Rust. |

### `modules/datasource/common › filterValidVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should filter out invalid versions | 136 | not-applicable | — | — | Renovate's shared release-result versioning filter is not implemented in Rust. |
| should use default versioning if none is specified | 144 | not-applicable | — | — | Renovate's shared release-result versioning filter is not implemented in Rust. |
| should use specified versioning if provided | 152 | not-applicable | — | — | Renovate's shared release-result versioning filter is not implemented in Rust. |

### `modules/datasource/common › sortAndRemoveDuplicates`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sorts releases by version and removes duplicates | 162 | not-applicable | — | — | Renovate's shared release-result sorting and deduplication helper is not implemented in Rust. |
| uses default versioning if none is specified | 183 | not-applicable | — | — | Renovate's shared release-result sorting and deduplication helper is not implemented in Rust. |

### `modules/datasource/common › applyConstraintsFiltering`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove constraints from releases if constraintsFiltering is not strict | 201 | not-applicable | — | — | Renovate's shared datasource constraints-filtering post-processor is not implemented in Rust. |
| should filter releases based on constraints if constraintsFiltering is strict | 230 | not-applicable | — | — | Renovate's shared datasource constraints-filtering post-processor is not implemented in Rust. |
| should return all releases when no configConstraints | 250 | not-applicable | — | — | Renovate's shared datasource constraints-filtering post-processor is not implemented in Rust. |
| should match exact constraints | 268 | not-applicable | — | — | Renovate's shared datasource constraints-filtering post-processor is not implemented in Rust. |
| should handle config with a range constraint, and a release with an exact version | 287 | not-applicable | — | — | Renovate's shared datasource constraints-filtering post-processor is not implemented in Rust. |
| should handle config with an exact version, and a release with a range constraint | 306 | not-applicable | — | — | Renovate's shared datasource constraints-filtering post-processor is not implemented in Rust. |
| should allow constraintsVersioning to override the datasource's default versioning | 325 | not-applicable | — | — | Renovate's shared datasource constraints-versioning override is not implemented in Rust. |

### `modules/datasource/common › applyVersionCompatibility`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns immediately if no versionCompatibility | 378 | not-applicable | — | — | Renovate's shared versionCompatibility filter is not implemented in Rust. |
| filters out non-matching | 383 | not-applicable | — | — | Renovate's shared versionCompatibility filter is not implemented in Rust. |
| filters out incompatible | 395 | not-applicable | — | — | Renovate's shared versionCompatibility filter is not implemented in Rust. |
| does not override versionOrig from extractVersion | 407 | not-applicable | — | — | Renovate's shared versionCompatibility and extractVersion interaction is not implemented in Rust. |

---


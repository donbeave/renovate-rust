# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/nuget/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/nuget/index.spec.ts
**Total tests:** 18 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/versioning/nuget/index › isSingleVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isSingleVersion("$input") === $expected | 5 | not-applicable | — | — | Renovate's NuGet exact-range single-version classifier is not implemented as a Rust API. |

### `modules/versioning/nuget/index › isStable()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$input") === $expected | 18 | ported | `nuget.rs` | `is_stable_matches_renovate_index_spec` | — |

### `modules/versioning/nuget/index › isValid()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 43 | not-applicable | — | — | Renovate's NuGet version/range validation and compatibility API is not implemented in Rust; Rust only exposes comparison, stability, and update-summary helpers. |

### `modules/versioning/nuget/index › isVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 118 | not-applicable | — | — | Renovate's NuGet standalone-version classifier is not implemented as a Rust API. |

### `modules/versioning/nuget/index › getMajor, getMinor, getPatch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input -> [$major, $minor, $patch] | 218 | not-applicable | — | — | Renovate's NuGet component accessor API is not implemented in Rust. |

### `modules/versioning/nuget/index › equals()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals($a, $b) === $expected | 258 | not-applicable | — | — | Renovate's NuGet equality helper is not implemented as a Rust API; equivalent valid-version comparison behavior is covered by `compare_matches_renovate_version_spec`. |

### `modules/versioning/nuget/index › isGreaterThan()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isGreaterThan($a, $b) === $expected | 303 | not-applicable | — | — | Renovate's NuGet greater-than helper is not implemented as a Rust API; equivalent valid-version comparison behavior is covered by `compare_matches_renovate_version_spec`. |

### `modules/versioning/nuget/index › isLessThanRange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isLessThanRange("$version", "$range") === $expected | 347 | not-applicable | — | — | Renovate's NuGet range parser and range comparison API is not implemented in Rust. |

### `modules/versioning/nuget/index › getSatisfyingVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getSatisfyingVersion($versions, $range) === "$expected" | 392 | not-applicable | — | — | Renovate's NuGet range parser and satisfying-version selector API is not implemented in Rust. |

### `modules/versioning/nuget/index › minSatisfyingVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| minSatisfyingVersion($versions, $range) === $expected | 420 | not-applicable | — | — | Renovate's NuGet range parser and minimum satisfying-version selector API is not implemented in Rust. |

### `modules/versioning/nuget/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a pinned value | 435 | not-applicable | — | — | Renovate's NuGet pinned-range value helper is not implemented in Rust. |

### `modules/versioning/nuget/index › getNewValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns newVersion if the range is version too | 441 | not-applicable | — | — | Renovate's NuGet update-value helper is not implemented in Rust. |
| returns null if version is invalid | 451 | not-applicable | — | — | Renovate's NuGet update-value helper and range validation path is not implemented in Rust. |
| returns null if range is invalid | 461 | not-applicable | — | — | Renovate's NuGet update-value helper and range validation path is not implemented in Rust. |

### `modules/versioning/nuget/index › getNewValue() › pin`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the new version | 472 | not-applicable | — | — | Renovate's NuGet pinned-range value helper is not implemented in Rust. |

### `modules/versioning/nuget/index › getNewValue() › bump`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| currentValue=$currentValue newVersion=$newVersion -> $expected | 478 | not-applicable | — | — | Renovate's NuGet range bumping helper is not implemented in Rust. |

### `modules/versioning/nuget/index › sortVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sortVersions($a, $b) === $expected | 522 | not-applicable | — | — | Renovate's NuGet sort callback API is not implemented in Rust; equivalent valid-version comparison behavior is covered by `compare_matches_renovate_version_spec`. |

### `modules/versioning/nuget/index › matches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 547 | not-applicable | — | — | Renovate's NuGet range parser and matcher API is not implemented in Rust. |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/nuget/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/nuget/index.spec.ts
**Total tests:** 18 | **Ported:** 1 | **Actionable:** 18 | **Status:** partial

### `modules/versioning/nuget/index › isSingleVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isSingleVersion("$input") === $expected | 5 | pending | — | — | — |

### `modules/versioning/nuget/index › isStable()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$input") === $expected | 18 | ported | `nuget.rs` | `is_stable_matches_renovate_index_spec` | — |

### `modules/versioning/nuget/index › isValid()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 43 | pending | — | — | — |

### `modules/versioning/nuget/index › isVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 118 | pending | — | — | — |

### `modules/versioning/nuget/index › getMajor, getMinor, getPatch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input -> [$major, $minor, $patch] | 218 | pending | — | — | — |

### `modules/versioning/nuget/index › equals()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals($a, $b) === $expected | 258 | pending | — | — | — |

### `modules/versioning/nuget/index › isGreaterThan()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isGreaterThan($a, $b) === $expected | 303 | pending | — | — | — |

### `modules/versioning/nuget/index › isLessThanRange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isLessThanRange("$version", "$range") === $expected | 347 | pending | — | — | — |

### `modules/versioning/nuget/index › getSatisfyingVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getSatisfyingVersion($versions, $range) === "$expected" | 392 | pending | — | — | — |

### `modules/versioning/nuget/index › minSatisfyingVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| minSatisfyingVersion($versions, $range) === $expected | 420 | pending | — | — | — |

### `modules/versioning/nuget/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a pinned value | 435 | pending | — | — | — |

### `modules/versioning/nuget/index › getNewValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns newVersion if the range is version too | 441 | pending | — | — | — |
| returns null if version is invalid | 451 | pending | — | — | — |
| returns null if range is invalid | 461 | pending | — | — | — |

### `modules/versioning/nuget/index › getNewValue() › pin`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the new version | 472 | pending | — | — | — |

### `modules/versioning/nuget/index › getNewValue() › bump`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| currentValue=$currentValue newVersion=$newVersion -> $expected | 478 | pending | — | — | — |

### `modules/versioning/nuget/index › sortVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sortVersions($a, $b) === $expected | 522 | pending | — | — | — |

### `modules/versioning/nuget/index › matches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 547 | pending | — | — | — |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/elm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/elm/index.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** pending

### `modules/versioning/elm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 5 | pending | — | — | — |
| isValid("$input") === $expected | 23 | pending | — | — | — |
| isSingleVersion("$input") === $expected | 43 | pending | — | — | — |
| isStable("$input") === $expected | 55 | pending | — | — | — |
| returns false for invalid version | 65 | pending | — | — | — |
| isCompatible("$input") === $expected | 71 | pending | — | — | — |
| extracts version components | 81 | pending | — | — | — |
| equals("$a", "$b") === $expected | 89 | pending | — | — | — |
| isGreaterThan("$a", "$b") === $expected | 100 | pending | — | — | — |
| sorts versions correctly | 112 | pending | — | — | — |
| matches("$version", "$range") === $expected | 120 | pending | — | — | — |
| returns false for invalid version | 139 | pending | — | — | — |
| returns false for invalid range | 143 | pending | — | — | — |
| returns false for malformed range where lower > upper | 147 | pending | — | — | — |
| isLessThanRange("$version", "$range") === $expected | 153 | pending | — | — | — |
| returns false for invalid version | 170 | pending | — | — | — |
| returns false for invalid range | 176 | pending | — | — | — |
| getSatisfyingVersion($versions, "$range") === $expected | 182 | pending | — | — | — |
| minSatisfyingVersion($versions, "$range") === $expected | 199 | pending | — | — | — |
| replaces exact version with new version | 215 | pending | — | — | — |
| handles bump strategy for exact version | 225 | pending | — | — | — |
| getNewValue("$currentValue", "$rangeStrategy", "$newVersion") === "$expected" | 237 | pending | — | — | — |
| returns null for invalid new version | 266 | pending | — | — | — |
| returns null for invalid current value | 276 | pending | — | — | — |
| returns null for unknown range strategy | 286 | pending | — | — | — |
| handles widen when newVersion equals upper bound exactly | 296 | pending | — | — | — |
| widens elm-version range for new compiler release | 307 | pending | — | — | — |
| keeps elm-version range unchanged when version is already satisfied | 318 | pending | — | — | — |
| replaces elm-version range when explicitly requested | 328 | pending | — | — | — |
| finds highest satisfying version for elm-version range | 341 | pending | — | — | — |
| returns null when no compiler version satisfies range | 355 | pending | — | — | — |

---


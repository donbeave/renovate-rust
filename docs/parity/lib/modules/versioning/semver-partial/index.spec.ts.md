# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/semver-partial/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/semver-partial/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `modules/versioning/semver-partial/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 5 | pending | — | — | — |
| isVersion("$version") === $expected | 24 | pending | — | — | — |
| isStable("$version") === $expected | 47 | pending | — | — | — |
| isSingleVersion("$version") === $expected | 70 | pending | — | — | — |
| matches("$version", "$range") === $expected | 87 | pending | — | — | — |
| should handle invalid range that is not ~latest or valid version | 141 | pending | — | — | — |
| getSatisfyingVersion($versions, "$range") === $expected | 149 | pending | — | — | — |
| minSatisfyingVersion($versions, "$range") === $expected | 185 | pending | — | — | — |
| isLessThanRange("$version", "$range") === $expected | 209 | pending | — | — | — |
| equals("$version", "$other") === $expected | 240 | pending | — | — | — |
| getMajor("$version") === $expected | 262 | pending | — | — | — |
| getMinor("$version") === $expected | 275 | pending | — | — | — |
| getPatch("$version") === $expected | 288 | pending | — | — | — |
| isGreaterThan("$version", "$other") === $expected | 301 | pending | — | — | — |
| sortVersions("$a", "$b") === $expected | 326 | pending | — | — | — |
| isBreaking("$version", "$current") === $expected | 348 | pending | — | — | — |
| isCompatible("$version") === $expected | 376 | pending | — | — | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 390 | pending | — | — | — |

---


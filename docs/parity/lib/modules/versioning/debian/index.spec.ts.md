# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/debian/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/debian/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** pending

### `modules/versioning/debian/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| test | 18 | pending | — | — | — |
| isValid("$version") === $expected | 22 | pending | — | — | — |
| isCompatible("$version") === $expected | 82 | pending | — | — | — |
| isSingleVersion("$version") === $expected | 104 | pending | — | — | — |
| isStable("$version") === $expected | 115 | pending | — | — | — |
| ensures that rolling release is not refreshed within frame time window: $version | 169 | not-applicable | — | — | Asserts expect(logger.debug).toHaveBeenCalledTimes(0) — logger spy infrastructure |
| isVersion("$version") === $expected | 188 | pending | — | — | — |
| getMajor, getMinor, getPatch for "$version" | 248 | pending | — | — | — |
| equals($a, $b) === $expected | 273 | pending | — | — | — |
| isGreaterThan("$a", "$b") === $expected | 297 | pending | — | — | — |
| getSatisfyingVersion($versions, "$range") === "$expected" | 340 | pending | — | — | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 361 | pending | — | — | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 383 | pending | — | — | — |
| debian.sortVersions($a, $b) === $expected | 409 | pending | — | — | — |
| matches("$version", "$range") === "$expected" | 429 | pending | — | — | — |
| checks runtime date handling & refresh rolling release data | 441 | not-applicable | — | — | Uses vi.setSystemTime + expect(logger.debug).toHaveBeenCalled — fake timers + logger spy |

---


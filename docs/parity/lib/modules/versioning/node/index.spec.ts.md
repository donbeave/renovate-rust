# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/node/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/node/index.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/node/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 14 | not-applicable | - | - | Renovate's Node.js versioning scheme, release codenames, and update-value helper are not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 43 | not-applicable | - | - | Renovate's Node.js LTS schedule and time-dependent stability classifier are not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 64 | not-applicable | - | - | Renovate's Node.js version, codename, and range validation is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 75 | not-applicable | - | - | Renovate's Node.js codename range matcher is not implemented as a Rust versioning API. |
| getSatisfyingVersion("$versions", "$range") === $expected | 87 | not-applicable | - | - | Renovate's Node.js codename satisfying-version selector is not implemented as a Rust versioning API. |
| minSatisfyingVersion("$versions", "$range") === $expected | 102 | not-applicable | - | - | Renovate's Node.js codename satisfying-version selector is not implemented as a Rust versioning API. |

---


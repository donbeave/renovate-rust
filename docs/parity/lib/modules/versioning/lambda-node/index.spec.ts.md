# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/lambda-node/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/lambda-node/index.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/lambda-node/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 47 | not-applicable | — | — | Renovate's Lambda Node runtime versioning scheme is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 71 | not-applicable | — | — | Renovate's Lambda Node stability classifier and schedule-data behavior are not implemented as a Rust versioning API. |
| isValid("$version") === $expected | 100 | not-applicable | — | — | Renovate's Lambda Node version validation is not implemented as a Rust versioning API. |
| matches("$version", "$range") === $expected | 112 | not-applicable | — | — | Renovate's Lambda Node range matcher is not implemented as a Rust versioning API. |
| getSatisfyingVersion("$versions", "$range") === $expected | 125 | not-applicable | — | — | Renovate's Lambda Node satisfying-version helper is not implemented as a Rust versioning API. |
| minSatisfyingVersion("$versions", "$range") === $expected | 139 | not-applicable | — | — | Renovate's Lambda Node satisfying-version helper is not implemented as a Rust versioning API. |

---


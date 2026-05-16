# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/maven/index.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/maven/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses same function module export and api object | 7 | not-applicable | — | — | Renovate's TypeScript module export identity contract has no Rust equivalent. |
| isValid("$version") === $expected | 11 | not-applicable | — | — | Renovate's full Maven `VersioningApi` validation contract is not implemented as a Rust API; Rust currently exposes a narrower comparator and update-summary helper. |
| isVersion("$version") === $expected | 32 | not-applicable | — | — | Renovate's full Maven version classifier is not implemented as a Rust API; Rust currently exposes a narrower comparator and update-summary helper. |
| isStable("$version") === $expected | 60 | not-applicable | — | — | Renovate's full Maven `VersioningApi` stability classifier is not implemented as a Rust API; Rust currently exposes a narrower comparator and update-summary helper. |
| "$input" is represented as [$major, $minor, $patch] | 89 | not-applicable | — | — | Renovate's Maven component parser is not implemented as a Rust API. |
| matches("$version", "$range") === $expected | 111 | not-applicable | — | — | Renovate's Maven range matcher is not implemented as a Rust API. |
| isGreaterThan("$a", "$b") === $expected | 158 | not-applicable | — | — | Renovate's full Maven `VersioningApi` comparator wrapper is not implemented as a Rust API; Rust currently exposes a narrower comparator and update-summary helper. |
| getSatisfyingVersion($versions, "$range") === $expected | 165 | not-applicable | — | — | Renovate's Maven satisfying-version helper is not implemented as a Rust API. |
| minSatisfyingVersion($versions, "$range") === $expected | 179 | not-applicable | — | — | Renovate's Maven satisfying-version helper is not implemented as a Rust API. |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 193 | not-applicable | — | — | Renovate's Maven update-value helper is not implemented as a Rust API. |
| matches("$version", "[2.164.0,2.165.0)") === $expected | 228 | not-applicable | — | — | Renovate's Maven range matcher for Jenkins-style ranges is not implemented as a Rust API. |
| matches("$version", "[2.164.0,2.165.0]") === $expected | 247 | not-applicable | — | — | Renovate's Maven range matcher for Jenkins-style ranges is not implemented as a Rust API. |
| matches("$version", "(,2.164.0)") === $expected | 266 | not-applicable | — | — | Renovate's Maven range matcher for Jenkins-style ranges is not implemented as a Rust API. |

---


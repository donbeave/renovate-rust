# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hashicorp/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hashicorp/index.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/hashicorp/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 4 | not-applicable | — | — | Renovate's full HashiCorp `VersioningApi` range matcher is not implemented as a Rust API; Rust currently exposes a narrower update-summary helper. |
| getSatisfyingVersion($versions, "$range") === $expected | 17 | not-applicable | — | — | Renovate's full HashiCorp `VersioningApi` satisfying-version helper is not implemented as a Rust API; Rust currently exposes a narrower update-summary helper. |
| isValid("$input") === $expected | 29 | not-applicable | — | — | Renovate's full HashiCorp `VersioningApi` validation contract is not implemented as a Rust API; Rust currently exposes a narrower update-summary helper. |
| isLessThanRange($version, $range) === $expected | 48 | not-applicable | — | — | Renovate's full HashiCorp `VersioningApi` range comparison helper is not implemented as a Rust API; Rust currently exposes a narrower update-summary helper. |
| minSatisfyingVersion($versions, "$range") === $expected | 59 | not-applicable | — | — | Renovate's full HashiCorp `VersioningApi` satisfying-version helper is not implemented as a Rust API; Rust currently exposes a narrower update-summary helper. |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 72 | not-applicable | — | — | Renovate's full HashiCorp update-value helper is not implemented as a Rust API; Rust currently exposes a narrower update-summary helper. |

---


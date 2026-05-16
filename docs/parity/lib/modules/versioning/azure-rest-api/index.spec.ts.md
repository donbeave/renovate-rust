# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/azure-rest-api/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/azure-rest-api/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/azure-rest-api/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | not-applicable | — | — | Renovate's Azure REST API date-versioning scheme is not implemented as a Rust versioning API. |
| isCompatible("$version") === $expected | 24 | not-applicable | — | — | Renovate's Azure REST API compatibility helper is not implemented as a Rust versioning API. |
| isStable("$version") === $expected | 32 | not-applicable | — | — | Renovate's Azure REST API stability classifier is not implemented as a Rust versioning API. |
| isSingleVersion("$version") === $expected | 44 | not-applicable | — | — | Renovate's Azure REST API single-version classifier is not implemented as a Rust versioning API. |
| isVersion("$version") === $expected | 52 | not-applicable | — | — | Renovate's Azure REST API version classifier is not implemented as a Rust versioning API. |
| getMajor("$version") === 1 | 64 | not-applicable | — | — | Renovate's Azure REST API numeric date component accessor is not implemented in Rust. |
| getMinor("$version") === 0 | 72 | not-applicable | — | — | Renovate's Azure REST API minor component accessor is not implemented in Rust. |
| getPatch("$version") === 0 | 80 | not-applicable | — | — | Renovate's Azure REST API patch component accessor is not implemented in Rust. |
| equals("$version", "$other") === $expected | 88 | not-applicable | — | — | Renovate's Azure REST API equality helper is not implemented in Rust. |
| isGreaterThan("$version", "$other") === $expected | 104 | not-applicable | — | — | Renovate's Azure REST API ordering comparator is not implemented in Rust. |
| sortVersions("$version", "$other") === $expected | 124 | not-applicable | — | — | Renovate's Azure REST API sort comparator is not implemented in Rust. |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/unity3d-packages/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/unity3d-packages/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/unity3d-packages/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | not-applicable | — | — | Renovate's Unity3D package versioning API is not implemented in Rust; Rust Unity support targets Unity editor version extraction and latest-LTS lookup. |
| isStable("$input") === $expected | 17 | not-applicable | — | — | Renovate's Unity3D package stability classifier is not implemented in Rust; Rust Unity support targets Unity editor version extraction and latest-LTS lookup. |
| equals($a, $b) === $expected | 29 | not-applicable | — | — | Renovate's Unity3D package equality helper is not implemented in Rust. |
| isGreaterThan($a, $b) === $expected | 41 | not-applicable | — | — | Renovate's Unity3D package ordering comparator is not implemented in Rust. |

---


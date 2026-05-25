# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/unity3d-packages/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/unity3d-packages/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `modules/versioning/unity3d-packages/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | ported | crates/renovate-core/src/versioning/unity3d_packages.rs | is_valid_matches_renovate_unity3d_packages_spec | — |
| isStable("$input") === $expected | 17 | ported | crates/renovate-core/src/versioning/unity3d_packages.rs | is_stable_matches_renovate_unity3d_packages_spec | — |
| equals($a, $b) === $expected | 29 | ported | crates/renovate-core/src/versioning/unity3d_packages.rs | equals_matches_renovate_unity3d_packages_spec | — |
| isGreaterThan($a, $b) === $expected | 41 | ported | crates/renovate-core/src/versioning/unity3d_packages.rs | is_greater_than_matches_renovate_unity3d_packages_spec | — |

---

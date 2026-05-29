# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/nixpkgs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/nixpkgs/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/versioning/nixpkgs/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 6 | ported | crates/renovate-core/src/versioning/nixpkgs.rs | is_valid_matches_renovate_nixpkgs_spec | — |
| isStable("$version") === $expected | 32 | ported | crates/renovate-core/src/versioning/nixpkgs.rs | is_stable_matches_renovate_nixpkgs_spec | — |
| equals($a, $b) === $expected | 50 | ported | crates/renovate-core/src/versioning/nixpkgs.rs | equals_matches_renovate_nixpkgs_spec | — |
| $versions -> sortVersions -> $expected | 62 | ported | crates/renovate-core/src/versioning/nixpkgs.rs | sort_versions_matches_renovate_nixpkgs_spec | — |
| equals($a, $b) === $expected | 73 | ported | crates/renovate-core/src/versioning/nixpkgs.rs | is_compatible_matches_renovate_nixpkgs_spec | — |

---


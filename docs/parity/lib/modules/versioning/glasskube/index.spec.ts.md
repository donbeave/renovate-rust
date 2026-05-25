# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/glasskube/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/glasskube/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** done

### `modules/versioning/glasskube/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$version") === $expected | 6 | ported | crates/renovate-core/src/versioning/glasskube.rs | is_stable_matches_renovate_glasskube_spec | — |
| isValid("$version") === $expected | 16 | ported | crates/renovate-core/src/versioning/glasskube.rs | is_valid_matches_renovate_glasskube_spec | — |
| getMajor, getMinor, getPatch for "$version" | 30 | ported | crates/renovate-core/src/versioning/glasskube.rs | get_components_matches_renovate_glasskube_spec | — |
| getMajor, getMinor, getPatch for "$version" | 44 | ported | crates/renovate-core/src/versioning/glasskube.rs | is_greater_than_matches_renovate_glasskube_spec | — |

---


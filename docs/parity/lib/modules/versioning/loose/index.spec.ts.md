# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/loose/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/loose/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `modules/versioning/loose/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$version") === $expected | 4 | ported | crates/renovate-core/src/versioning/loose.rs | is_version_matches_renovate_loose_spec | — |
| isValid("$version") === $expected | 13 | ported | crates/renovate-core/src/versioning/loose.rs | is_valid_matches_renovate_loose_spec | — |
| equals("$a", "$b") === $expected | 41 | ported | crates/renovate-core/src/versioning/loose.rs | equals_matches_renovate_loose_spec | — |
| isGreaterThan("$a", "$b") === $expected | 52 | ported | crates/renovate-core/src/versioning/loose.rs | is_greater_than_matches_renovate_loose_spec | — |
| isCompatible("$version") === $expected | 72 | ported | crates/renovate-core/src/versioning/loose.rs | is_compatible_matches_renovate_loose_spec | — |
| isSingleVersion("$version") === $expected | 79 | ported | crates/renovate-core/src/versioning/loose.rs | is_single_version_matches_renovate_loose_spec | — |

---

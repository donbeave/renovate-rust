# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/perl/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/perl/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** done

### `modules/versioning/perl/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | ported | crates/renovate-core/src/versioning/perl.rs | is_valid_matches_renovate_perl_spec | — |
| isStable("$input") === $expected | 29 | ported | crates/renovate-core/src/versioning/perl.rs | is_stable_matches_renovate_perl_spec | — |
| equals($a, $b) === $expected | 43 | ported | crates/renovate-core/src/versioning/perl.rs | equals_matches_renovate_perl_spec | — |
| isGreaterThan($a, $b) === $expected | 57 | ported | crates/renovate-core/src/versioning/perl.rs | is_greater_than_matches_renovate_perl_spec | — |

---


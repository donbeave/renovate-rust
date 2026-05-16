# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/rpm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/rpm/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `modules/versioning/rpm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 3 | ported | `rpm.rs` | `is_valid_matches_renovate_rpm_spec` | — |
| equals("$a", "$b") === $expected | 62 | ported | `rpm.rs` | `equals_matches_renovate_rpm_spec` | — |
| isGreaterThan("$a", "$b") === $expected | 96 | ported | `rpm.rs` | `is_greater_than_matches_renovate_rpm_spec` | — |
| getMajor("$version") === $expected | 154 | ported | `rpm.rs` | `get_major_matches_renovate_rpm_spec` | — |
| getMinor("$version") === $expected | 168 | ported | `rpm.rs` | `get_minor_matches_renovate_rpm_spec` | — |
| getPatch("$version") === $expected | 182 | ported | `rpm.rs` | `get_patch_matches_renovate_rpm_spec` | — |

---


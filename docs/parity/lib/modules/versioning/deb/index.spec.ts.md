# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/deb/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/deb/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `modules/versioning/deb/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | ported | `versioning/deb.rs` | `deb_is_valid` | — |
| equals("$a", "$b") === $expected | 60 | ported | `versioning/deb.rs` | `deb_equals` | — |
| isGreaterThan("$a", "$b") === $expected | 84 | ported | `versioning/deb.rs` | `deb_is_greater_than` | — |
| isSingleVersion("$version") === $expected | 128 | ported | `versioning/deb.rs` | `deb_is_single_version` | — |
| getMajor("$version") === $expected | 136 | ported | `versioning/deb.rs` | `deb_get_major` | — |
| getMinor("$version") === $expected | 149 | ported | `versioning/deb.rs` | `deb_get_minor` | — |
| getPatch("$version") === $expected | 162 | ported | `versioning/deb.rs` | `deb_get_patch` | — |

---

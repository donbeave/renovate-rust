# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/devbox/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/devbox/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `modules/versioning/devbox/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$version") === $expected | 4 | ported | crates/renovate-core/src/versioning/devbox.rs | is_version_matches_renovate_devbox_spec | — |
| isValid("$version") === $isValid | 34 | ported | crates/renovate-core/src/versioning/devbox.rs | is_valid_matches_renovate_devbox_spec | — |
| matches("$version", "$range") === $expected | 64 | ported | crates/renovate-core/src/versioning/devbox.rs | matches_matches_renovate_devbox_spec | — |
| equals("$version", "$range") === $expected | 84 | ported | crates/renovate-core/src/versioning/devbox.rs | equals_matches_renovate_devbox_spec | — |

---


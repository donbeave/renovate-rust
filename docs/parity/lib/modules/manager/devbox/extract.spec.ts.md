# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/devbox/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devbox/extract.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when the devbox JSON file is empty | 6 | ported | `devbox.rs` | `empty_returns_empty` | — |
| returns null when the devbox JSON file is malformed | 11 | ported | `devbox.rs` | `invalid_json_returns_empty` | — |
| returns null when the devbox JSON file has no packages | 16 | ported | `devbox.rs` | `no_packages_key_returns_empty` | — |
| returns a package dependency when the devbox JSON file has a single package | 21 | ported | `devbox.rs` | `array_form` | — |
| returns a package dependency when the devbox JSON file has a single package with a version object | 42 | ported | `devbox.rs` | `object_with_version_field` | — |
| returns invalid-version when the devbox JSON file has a single package with an invalid version | 65 | ported | `devbox.rs` | `invalid_semver_range_flagged` | — |
| returns a package dependency when the devbox JSON file has multiple packages | 89 | ported | `devbox.rs` | `valid_versions_have_no_skip_reason` | — |
| returns a package dependency when the devbox JSON file has multiple packages with in a packages object | 115 | ported | `devbox.rs` | `object_form` | — |
| returns a package dependency when the devbox JSON file has multiple packages with package objects | 144 | ported | `devbox.rs` | `object_form_with_nested_version` | — |
| returns invalid dependencies | 177 | ported | `devbox.rs` | `mixed_valid_and_invalid_versions` | — |
| returns invalid dependencies with package objects | 213 | ported | `devbox.rs` | `object_form_mixed_valid_invalid` | — |
| returns invalid dependencies from the packages array | 251 | ported | `devbox.rs` | `array_form_with_invalid_and_no_version` | — |
| returns null if there are no dependencies | 288 | ported | `devbox.rs` | `empty_packages_array_returns_empty` | — |

---


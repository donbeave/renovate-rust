# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/bazel-module/bzlmod-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/bazel-module/bzlmod-version.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 0 | **Status:** done

### `modules/versioning/bazel-module/bzlmod-version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when all digits | 5 | ported | `bazel_module/bzlmod_version.rs` | `identifier_when_all_digits` | — |
| when not all digits | 12 | ported | `bazel_module/bzlmod_version.rs` | `identifier_when_not_all_digits` | — |
| $a equals $b | 19 | ported | `bazel_module/bzlmod_version.rs` | `identifier_equals_table` | — |
| $a is isLessThan $b | 32 | ported | `bazel_module/bzlmod_version.rs` | `identifier_is_less_than_table` | — |
| VersionPart.create(...$a} | 49 | ported | `bazel_module/bzlmod_version.rs` | `version_part_create` | — |
| .asString | 59 | ported | `bazel_module/bzlmod_version.rs` | `version_part_as_string` | — |
| .major | 68 | ported | `bazel_module/bzlmod_version.rs` | `version_part_major` | — |
| .minor | 78 | ported | `bazel_module/bzlmod_version.rs` | `version_part_minor` | — |
| .patch | 87 | ported | `bazel_module/bzlmod_version.rs` | `version_part_patch` | — |
| $a equals $b | 96 | ported | `bazel_module/bzlmod_version.rs` | `version_part_equals_table` | — |
| $a is isLessThan $b | 107 | ported | `bazel_module/bzlmod_version.rs` | `version_part_is_less_than_table` | — |
| .isEmpty | 125 | ported | `bazel_module/bzlmod_version.rs` | `version_part_is_empty` | — |
| constructor($v) | 137 | ported | `bazel_module/bzlmod_version.rs` | `bzlmod_version_constructor` | — |
| bad versions $a | 153 | ported | `bazel_module/bzlmod_version.rs` | `bzlmod_version_bad_versions` | — |
| $a equals $b | 168 | ported | `bazel_module/bzlmod_version.rs` | `bzlmod_version_equals_table` | — |
| $a is isLessThan $b | 188 | ported | `bazel_module/bzlmod_version.rs` | `bzlmod_version_is_less_than_table` | — |
| $a isGreaterThan $b | 208 | ported | `bazel_module/bzlmod_version.rs` | `bzlmod_version_is_greater_than_table` | — |
| defaultCompare($a, $b) | 221 | ported | `bazel_module/bzlmod_version.rs` | `bzlmod_version_default_compare_table` | — |

---


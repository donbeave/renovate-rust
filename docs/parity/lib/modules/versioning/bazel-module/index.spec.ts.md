# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/bazel-module/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/bazel-module/index.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 0 | **Status:** done

### `modules/versioning/bazel-module/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getMajor() | 5 | ported | `versioning/bazel_module.rs` | `bzlmod_get_major` | — |
| getMinor() | 9 | ported | `versioning/bazel_module.rs` | `bzlmod_get_minor` | — |
| getPatch() | 13 | ported | `versioning/bazel_module.rs` | `bzlmod_get_patch` | — |
| equals($a, $b) | 17 | ported | `versioning/bazel_module.rs` | `bzlmod_equals` | — |
| isGreaterThan($a, $b) | 27 | ported | `versioning/bazel_module.rs` | `bzlmod_is_greater_than` | — |
| isLessThanRange($a, $b) | 36 | ported | `versioning/bazel_module.rs` | `bzlmod_is_less_than_range` | — |
| getSatisfyingVersion(vers, rng) | 45 | ported | `versioning/bazel_module.rs` | `bzlmod_get_satisfying_version` | — |
| sortVersions($a, $b) | 56 | ported | `versioning/bazel_module.rs` | `bzlmod_sort_versions` | — |
| isStable | 65 | ported | `versioning/bazel_module.rs` | `bzlmod_is_stable` | — |
| isValid($a) | 74 | ported | `versioning/bazel_module.rs` | `bzlmod_is_valid` | — |
| isVersion($a) | 90 | ported | `versioning/bazel_module.rs` | `bzlmod_is_version_null` | — |
| getNewValue() | 100 | ported | `versioning/bazel_module.rs` | `bzlmod_get_new_value` | — |

---

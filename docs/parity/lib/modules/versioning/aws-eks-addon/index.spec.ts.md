# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/aws-eks-addon/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/aws-eks-addon/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/versioning/aws-eks-addon/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return 1.23.7 and release version | 5 | ported | crates/renovate-core/src/versioning/aws_eks_addon.rs | get_major_minor_patch_matches_renovate_aws_eks_addon_index_spec | — |
| isValid("$input") === $expected | 13 | ported | crates/renovate-core/src/versioning/aws_eks_addon.rs | is_valid_matches_renovate_aws_eks_addon_index_spec | — |
| isValid("$input") === $expected | 41 | ported | crates/renovate-core/src/versioning/aws_eks_addon.rs | is_version_matches_renovate_aws_eks_addon_index_spec | — |
| isCompatible("$input") === $expected | 67 | ported | crates/renovate-core/src/versioning/aws_eks_addon.rs | is_compatible_single_arg_matches_renovate_aws_eks_addon_index_spec | — |
| isCompatible($version, $current) === $expected | 91 | ported | crates/renovate-core/src/versioning/aws_eks_addon.rs | is_compatible_two_args_matches_renovate_aws_eks_addon_index_spec | — |
| isGreaterThan($version, $other) === $expected | 110 | ported | crates/renovate-core/src/versioning/aws_eks_addon.rs | is_greater_than_matches_renovate_aws_eks_addon_index_spec | — |
| getSatisfyingVersion | 129 | ported | crates/renovate-core/src/versioning/aws_eks_addon.rs | get_satisfying_version_matches_renovate_aws_eks_addon_index_spec | — |

---


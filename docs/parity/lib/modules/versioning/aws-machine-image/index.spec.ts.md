# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/aws-machine-image/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/aws-machine-image/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/versioning/aws-machine-image/index › parse(version)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return 1.0.0 | 5 | ported | `aws_machine_image.rs` | `parse_returns_fixed_components` | — |

### `modules/versioning/aws-machine-image/index › isValid(version)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 13 | ported | `aws_machine_image.rs` | `is_valid_returns_true_for_ami_id` | — |
| should return false | 17 | ported | `aws_machine_image.rs` | `is_valid_returns_false_for_short_ami_id` | — |

### `modules/versioning/aws-machine-image/index › isVersion(version)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 23 | ported | `aws_machine_image.rs` | `is_version_returns_true_for_ami_id` | — |
| should return false | 27 | ported | `aws_machine_image.rs` | `is_version_returns_false_for_short_ami_id` | — |

### `modules/versioning/aws-machine-image/index › isCompatible(version)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 33 | ported | `aws_machine_image.rs` | `is_compatible_returns_true_for_ami_id_without_range` | — |
| should return false | 37 | ported | `aws_machine_image.rs` | `is_compatible_returns_false_for_short_ami_id_without_range` | — |

### `modules/versioning/aws-machine-image/index › isCompatible(version,range)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 43 | ported | `aws_machine_image.rs` | `is_compatible_returns_true_for_ami_id_with_range` | — |
| should return false | 51 | ported | `aws_machine_image.rs` | `is_compatible_returns_false_for_short_ami_id_with_range` | — |

### `modules/versioning/aws-machine-image/index › isGreaterThan(version1, version2)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 58 | ported | `aws_machine_image.rs` | `is_greater_than_returns_true_for_any_ami_pair` | — |

---


# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/lockfile/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/lockfile/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns artifact error | 36 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| returns null if no .terraform.lock.hcl found | 56 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| returns null if .terraform.lock.hcl is empty | 67 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| returns null if .terraform.lock.hcl is invalid | 81 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update single dependency with exact constraint and depType provider | 95 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update single dependency with exact constraint and and depType required_provider | 151 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| does not update dependency with exact constraint during lockfile update | 209 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| does not update dependency with exact constraint within multiple during lockfile update | 249 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| do not update dependency with depType module | 289 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update single dependency with range constraint and minor update from private registry | 307 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update single dependency with range constraint and major update | 366 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update single dependency in subfolder | 424 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update multiple dependencies which are not ordered | 484 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| do full lock file maintenance | 621 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| do full lock file maintenance with lockfile in subfolder | 757 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| do full lock file maintenance without necessary changes | 873 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| return null if hashing fails | 933 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| return null if experimental flag is not set | 1023 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| preserves constraints when current value and new value are same | 1037 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| replaces current value to new version within a constraint | 1097 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| replaces current version to new version within a constraint | 1157 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |

### `getNewConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| correctly calculate new constraint on pinning | 1217 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update constraint with multiple elements | 1230 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update constraint when current version is matched multiple times | 1243 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| update constraint when current version is in a complicated constraint | 1256 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |
| create constraint with full version | 1269 | not-applicable | — | — | tests Terraform lock file management requiring Terraform provider registry calls |

---


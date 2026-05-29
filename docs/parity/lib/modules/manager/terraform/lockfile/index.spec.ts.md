# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/lockfile/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/lockfile/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns artifact error | 36 | pending | — | — | —|
| returns null if no .terraform.lock.hcl found | 56 | pending | — | — | —|
| returns null if .terraform.lock.hcl is empty | 67 | pending | — | — | —|
| returns null if .terraform.lock.hcl is invalid | 81 | pending | — | — | —|
| update single dependency with exact constraint and depType provider | 95 | pending | — | — | —|
| update single dependency with exact constraint and and depType required_provider | 151 | pending | — | — | —|
| does not update dependency with exact constraint during lockfile update | 209 | pending | — | — | —|
| does not update dependency with exact constraint within multiple during lockfile update | 249 | pending | — | — | —|
| do not update dependency with depType module | 289 | pending | — | — | —|
| update single dependency with range constraint and minor update from private registry | 307 | pending | — | — | —|
| update single dependency with range constraint and major update | 366 | pending | — | — | —|
| update single dependency in subfolder | 424 | pending | — | — | —|
| update multiple dependencies which are not ordered | 484 | pending | — | — | —|
| do full lock file maintenance | 621 | pending | — | — | —|
| do full lock file maintenance with lockfile in subfolder | 757 | pending | — | — | —|
| do full lock file maintenance without necessary changes | 873 | pending | — | — | —|
| return null if hashing fails | 933 | pending | — | — | —|
| return null if experimental flag is not set | 1023 | pending | — | — | —|
| preserves constraints when current value and new value are same | 1037 | pending | — | — | —|
| replaces current value to new version within a constraint | 1097 | pending | — | — | —|
| replaces current version to new version within a constraint | 1157 | pending | — | — | —|

### `getNewConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| correctly calculate new constraint on pinning | 1217 | pending | — | — | —|
| update constraint with multiple elements | 1230 | pending | — | — | —|
| update constraint when current version is matched multiple times | 1243 | pending | — | — | —|
| update constraint when current version is in a complicated constraint | 1256 | pending | — | — | —|
| create constraint with full version | 1269 | pending | — | — | —|

---


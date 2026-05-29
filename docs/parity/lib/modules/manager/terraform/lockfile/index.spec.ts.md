# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/lockfile/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/lockfile/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns artifact error | 36 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| returns null if no .terraform.lock.hcl found | 56 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| returns null if .terraform.lock.hcl is empty | 67 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| returns null if .terraform.lock.hcl is invalid | 81 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update single dependency with exact constraint and depType provider | 95 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update single dependency with exact constraint and and depType required_provider | 151 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| does not update dependency with exact constraint during lockfile update | 209 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| does not update dependency with exact constraint within multiple during lockfile update | 249 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| do not update dependency with depType module | 289 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update single dependency with range constraint and minor update from private registry | 307 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update single dependency with range constraint and major update | 366 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update single dependency in subfolder | 424 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update multiple dependencies which are not ordered | 484 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| do full lock file maintenance | 621 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| do full lock file maintenance with lockfile in subfolder | 757 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| do full lock file maintenance without necessary changes | 873 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| return null if hashing fails | 933 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| return null if experimental flag is not set | 1023 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| preserves constraints when current value and new value are same | 1037 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| replaces current value to new version within a constraint | 1097 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| replaces current version to new version within a constraint | 1157 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|

### `getNewConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| correctly calculate new constraint on pinning | 1217 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update constraint with multiple elements | 1230 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update constraint when current version is matched multiple times | 1243 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| update constraint when current version is in a complicated constraint | 1256 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|
| create constraint with full version | 1269 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Terraform lockfile update pipeline|

---


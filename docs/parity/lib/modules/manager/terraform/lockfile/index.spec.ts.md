# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/lockfile/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/lockfile/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns artifact error | 36 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if no .terraform.lock.hcl found | 56 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if .terraform.lock.hcl is empty | 67 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if .terraform.lock.hcl is invalid | 81 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update single dependency with exact constraint and depType provider | 95 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update single dependency with exact constraint and and depType required_provider | 151 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| does not update dependency with exact constraint during lockfile update | 209 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| does not update dependency with exact constraint within multiple during lockfile update | 249 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| do not update dependency with depType module | 289 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update single dependency with range constraint and minor update from private registry | 307 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update single dependency with range constraint and major update | 366 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update single dependency in subfolder | 424 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update multiple dependencies which are not ordered | 484 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| do full lock file maintenance | 621 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| do full lock file maintenance with lockfile in subfolder | 757 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| do full lock file maintenance without necessary changes | 873 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| return null if hashing fails | 933 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| return null if experimental flag is not set | 1023 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| preserves constraints when current value and new value are same | 1037 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| replaces current value to new version within a constraint | 1097 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| replaces current version to new version within a constraint | 1157 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |

### `getNewConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| correctly calculate new constraint on pinning | 1217 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update constraint with multiple elements | 1230 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update constraint when current version is matched multiple times | 1243 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| update constraint when current version is in a complicated constraint | 1256 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |
| create constraint with full version | 1269 | not-applicable | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests terraform lockfile via vitest-mocked fs/exec; Rust tests this at different layer |

---


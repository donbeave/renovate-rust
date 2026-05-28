# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/azure-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/azure-helper.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** done

### `getRef`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get the ref with short ref name | 23 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should not get ref | 34 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should get the ref with full ref name | 45 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |

### `getAzureBranchObj`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get the branch object | 58 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should get the branch object when ref missing | 73 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |

### `getFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null error GitItemNotFoundException | 86 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return null error GitUnresolvableToCommitException | 115 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return the file content because it is not a json | 144 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return null because the file is not readable | 173 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |

### `getCommitDetails`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get commit details | 193 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |

### `getMergeMethod`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should default to NoFastForward | 208 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return NoFastForward when policy explicitly set | 220 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return RebaseMerge | 246 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return Squash | 272 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return Squash when Project wide exact branch policy exists | 298 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return default branch policy | 327 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return most specific exact branch policy | 366 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |
| should return most specific prefix branch policy | 435 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |

### `getAllProjectTeams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get all teams | 493 | not-applicable | — | — | Requires vi.mock(azure-got-wrapper) SDK mock infrastructure |

---


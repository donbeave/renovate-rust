# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/onboarding-branch-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/onboarding-branch-cache.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** done

### `workers/repository/onboarding/branch/onboarding-branch-cache › setOnboardingCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not create new cache | 24 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| sets new cache | 31 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| updates old cache | 45 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

### `workers/repository/onboarding/branch/onboarding-branch-cache › deleteOnboardingCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes cache | 68 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

### `workers/repository/onboarding/branch/onboarding-branch-cache › hasOnboardingBranchChanged()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return true if cache is absent | 84 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| returns true | 92 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| returns false | 108 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| returns false when branch is modified but has not changed since last run | 124 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

### `workers/repository/onboarding/branch/onboarding-branch-cache › isOnboardingBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| falls back to git if cache is absent | 142 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| falls back to git if onboarding branch is updated | 153 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| returns cached value | 172 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

### `workers/repository/onboarding/branch/onboarding-branch-cache › isOnboardingBranchConflicted()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| falls back to git if cache is absent | 192 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| falls back to git if default branch is updated | 203 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| falls back to git if onboarding branch is modified | 222 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| returns cached value | 241 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

### `workers/repository/onboarding/branch/onboarding-branch-cache › getOnboardingFileNameFromCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 261 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| returns undefined | 271 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

### `workers/repository/onboarding/branch/onboarding-branch-cache › getOnboardingConfigFromCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 278 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |
| returns undefined | 288 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

### `workers/repository/onboarding/branch/onboarding-branch-cache › setOnboardingConfigDetails()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 295 | not-applicable | — | — | Requires vi.mock repository/cache mock infrastructure |

---

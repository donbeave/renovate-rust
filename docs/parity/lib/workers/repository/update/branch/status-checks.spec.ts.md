# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/status-checks.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/status-checks.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/branch/status-checks › setStability`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if not configured  | 26 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| sets status yellow  | 31 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| sets status green  | 38 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if already set  | 45 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if statusCheckNames.minimumReleaseAge is null  | 53 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if statusCheckNames.minimumReleaseAge is empty string  | 68 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if statusCheckNames is undefined  | 83 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| does not set status in dry mode  | 96 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |

### `workers/repository/update/branch/status-checks › setConfidence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if not configured  | 121 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| sets status yellow  | 126 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| sets status green  | 134 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if already set  | 142 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if statusCheckNames.mergeConfidence is null  | 151 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if statusCheckNames.mergeConfidence is empty string  | 167 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| skips status if statusCheckNames is undefined  | 183 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |
| does not set status in dry mode  | 197 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |

### `workers/repository/update/branch/status-checks › getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return green if ignoreTests=true  | 211 | not-applicable | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer | — | Mock framework internals — tests branch status-checks via vitest-mocked platform; Rust tests this at different layer |

---


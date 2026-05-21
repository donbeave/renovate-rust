# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/status-checks.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/status-checks.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 17 | **Status:** not-applicable

### `workers/repository/update/branch/status-checks › setStability`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if not configured | 26 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| sets status yellow | 31 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| sets status green | 38 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if already set | 45 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if statusCheckNames.minimumReleaseAge is null | 53 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if statusCheckNames.minimumReleaseAge is empty string | 68 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if statusCheckNames is undefined | 83 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| does not set status in dry mode | 96 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |

### `workers/repository/update/branch/status-checks › setConfidence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if not configured | 121 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| sets status yellow | 126 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| sets status green | 134 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if already set | 142 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if statusCheckNames.mergeConfidence is null | 151 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if statusCheckNames.mergeConfidence is empty string | 167 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| skips status if statusCheckNames is undefined | 183 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |
| does not set status in dry mode | 197 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |

### `workers/repository/update/branch/status-checks › getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return green if ignoreTests=true | 211 | not-applicable | — | — | tests branch status check creation via platform API; platform interactions out of scope |

---


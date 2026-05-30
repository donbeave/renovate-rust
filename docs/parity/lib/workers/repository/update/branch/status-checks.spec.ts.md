# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/status-checks.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/status-checks.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 17 | **Status:** pending-applicable

### `workers/repository/update/branch/status-checks › setStability`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if not configured | 26 | pending | — | — | — |
| sets status yellow | 31 | pending | — | — | — |
| sets status green | 38 | pending | — | — | — |
| skips status if already set | 45 | pending | — | — | — |
| skips status if statusCheckNames.minimumReleaseAge is null | 53 | pending | — | — | — |
| skips status if statusCheckNames.minimumReleaseAge is empty string | 68 | pending | — | — | — |
| skips status if statusCheckNames is undefined | 83 | pending | — | — | — |
| does not set status in dry mode | 96 | pending | — | — | — |

### `workers/repository/update/branch/status-checks › setConfidence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if not configured | 121 | pending | — | — | — |
| sets status yellow | 126 | pending | — | — | — |
| sets status green | 134 | pending | — | — | — |
| skips status if already set | 142 | pending | — | — | — |
| skips status if statusCheckNames.mergeConfidence is null | 151 | pending | — | — | — |
| skips status if statusCheckNames.mergeConfidence is empty string | 167 | pending | — | — | — |
| skips status if statusCheckNames is undefined | 183 | pending | — | — | — |
| does not set status in dry mode | 197 | pending | — | — | — |

### `workers/repository/update/branch/status-checks › getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return green if ignoreTests=true | 211 | pending | — | — | — |

---


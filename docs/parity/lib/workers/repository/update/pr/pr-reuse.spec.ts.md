# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/pr-reuse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/pr-reuse.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/pr/pr-reuse`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if platform does not support PR reuse | 18 | not-applicable | — | — | Platform-specific PR reuse logic |
| returns null if PR is not found | 29 | not-applicable | — | — | Platform-specific PR reuse logic |
| returns null if PR title does not seem to be autoclosed | 37 | not-applicable | — | — | Platform-specific PR reuse logic |
| returns null if closedAt field is absent | 50 | not-applicable | — | — | Platform-specific PR reuse logic |
| returns null if it was closed long time ago | 63 | not-applicable | — | — | Platform-specific PR reuse logic |
| returns null for dry-runs | 77 | not-applicable | — | — | Platform-specific PR reuse logic |
| returns updated Pr after successful reopen | 94 | not-applicable | — | — | Platform-specific PR reuse logic |
| returns null if the retry throws | 130 | not-applicable | — | — | Platform-specific PR reuse logic |

---


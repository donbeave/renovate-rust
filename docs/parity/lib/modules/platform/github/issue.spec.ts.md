# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/issue.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/issue.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 7 | **Status:** done

### `GithubIssueCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty cache | 16 | not-applicable | — | — | Requires memCache + repository cache mock infrastructure |
| stores issues to the cache | 20 | not-applicable | — | — | Requires memCache + repository cache mock infrastructure |
| returns issues from the cache in the correct order | 64 | not-applicable | — | — | Requires memCache + repository cache mock infrastructure |
| updates particular issue in the cache | 120 | not-applicable | — | — | Requires memCache + repository cache mock infrastructure |
| removes particular issue from the cache | 162 | not-applicable | — | — | Requires memCache + repository cache mock infrastructure |
| reconciles cache | 188 | not-applicable | — | — | Requires memCache + repository cache mock infrastructure |
| resets cache during failed reconciliation | 246 | not-applicable | — | — | Requires memCache + repository cache mock infrastructure |

---


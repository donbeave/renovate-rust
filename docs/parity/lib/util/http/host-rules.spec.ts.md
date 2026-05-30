# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/host-rules.spec.ts
**Total tests:** 28 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/host-rules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds token | 63 | not-applicable | — | — | — |
| adds token to an api.github.com URL | 78 | not-applicable | — | — | — |
| adds auth | 95 | not-applicable | — | — | — |
| adds custom auth | 108 | not-applicable | — | — | — |
| skips | 126 | not-applicable | — | — | — |
| uses http2 | 138 | not-applicable | — | — | — |
| uses http keep-alive | 154 | not-applicable | — | — | — |
| disables http2 | 166 | not-applicable | — | — | — |
| noAuth | 183 | not-applicable | — | — | — |
| certificateAuthority | 195 | not-applicable | — | — | — |
| privateKey | 216 | not-applicable | — | — | — |
| certificate | 237 | not-applicable | — | — | — |
| no fallback to github | 258 | not-applicable | — | — | — |
| fallback to github | 332 | not-applicable | — | — | — |
| when multiple GitHub host types are set | 461 | not-applicable | — | — | — |

### `util/http/host-rules › GHE platform endpoint fallback`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fallback to github for non-listed hostType targeting GHE endpoint | 587 | not-applicable | — | — | — |
| no fallback when request targets a different host | 609 | not-applicable | — | — | — |
| no fallback to gitlab | 620 | not-applicable | — | — | — |
| fallback to gitlab | 677 | not-applicable | — | — | — |
| no fallback to bitbucket | 734 | not-applicable | — | — | — |
| fallback to bitbucket | 753 | not-applicable | — | — | — |
| no fallback to bitbucket-server | 768 | not-applicable | — | — | — |
| fallback to bitbucket-server | 787 | not-applicable | — | — | — |
| no fallback to gitea | 802 | not-applicable | — | — | — |
| fallback to gitea | 822 | not-applicable | — | — | — |
| should remove forbidden headers from request | 835 | not-applicable | — | — | — |
| should replace existing headers with host rule headers | 852 | not-applicable | — | — | — |
| enabled=false with noAuth | 872 | not-applicable | — | — | — |

---

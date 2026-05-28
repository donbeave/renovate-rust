# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/host-rules.spec.ts
**Total tests:** 28 | **Ported:** 0 | **Actionable:** 28 | **Status:** not-applicable

### `util/http/host-rules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds token | 63 | not-applicable | — | — | Uses vi.mock('global-agent') — all tests depend on global-agent proxy mock infrastructure |
| adds token to an api.github.com URL | 78 | not-applicable | — | — | Uses vi.mock('global-agent') |
| adds auth | 95 | not-applicable | — | — | Uses vi.mock('global-agent') |
| adds custom auth | 108 | not-applicable | — | — | Uses vi.mock('global-agent') |
| skips | 126 | not-applicable | — | — | Uses vi.mock('global-agent') |
| uses http2 | 138 | not-applicable | — | — | Uses vi.mock('global-agent') |
| uses http keep-alive | 154 | not-applicable | — | — | Uses vi.mock('global-agent') |
| disables http2 | 166 | not-applicable | — | — | Uses vi.mock('global-agent') |
| noAuth | 183 | not-applicable | — | — | Uses vi.mock('global-agent') |
| certificateAuthority | 195 | not-applicable | — | — | Uses vi.mock('global-agent') |
| privateKey | 216 | not-applicable | — | — | Uses vi.mock('global-agent') |
| certificate | 237 | not-applicable | — | — | Uses vi.mock('global-agent') |
| no fallback to github | 258 | not-applicable | — | — | Uses vi.mock('global-agent') |
| fallback to github | 332 | not-applicable | — | — | Uses vi.mock('global-agent') |
| when multiple GitHub host types are set | 461 | not-applicable | — | — | Uses vi.mock('global-agent') |

### `util/http/host-rules › GHE platform endpoint fallback`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fallback to github for non-listed hostType targeting GHE endpoint | 587 | not-applicable | — | — | Uses vi.mock('global-agent') |
| no fallback when request targets a different host | 609 | not-applicable | — | — | Uses vi.mock('global-agent') |
| no fallback to gitlab | 620 | not-applicable | — | — | Uses vi.mock('global-agent') |
| fallback to gitlab | 677 | not-applicable | — | — | Uses vi.mock('global-agent') |
| no fallback to bitbucket | 734 | not-applicable | — | — | Uses vi.mock('global-agent') |
| fallback to bitbucket | 753 | not-applicable | — | — | Uses vi.mock('global-agent') |
| no fallback to bitbucket-server | 768 | not-applicable | — | — | Uses vi.mock('global-agent') |
| fallback to bitbucket-server | 787 | not-applicable | — | — | Uses vi.mock('global-agent') |
| no fallback to gitea | 802 | not-applicable | — | — | Uses vi.mock('global-agent') |
| fallback to gitea | 822 | not-applicable | — | — | Uses vi.mock('global-agent') |
| should remove forbidden headers from request | 835 | not-applicable | — | — | Uses vi.mock('global-agent') |
| should replace existing headers with host rule headers | 852 | not-applicable | — | — | Uses vi.mock('global-agent') |
| enabled=false with noAuth | 872 | not-applicable | — | — | Uses vi.mock('global-agent') |

---

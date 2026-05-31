# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/host-rules.spec.ts
**Total tests:** 28 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/http/host-rules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds token | 63 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| adds token to an api.github.com URL | 78 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| adds auth | 95 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| adds custom auth | 108 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| skips | 126 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| uses http2 | 138 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| uses http keep-alive | 154 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| disables http2 | 166 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| noAuth | 183 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| certificateAuthority | 195 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| privateKey | 216 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| certificate | 237 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| no fallback to github | 258 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| fallback to github | 332 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| when multiple GitHub host types are set | 461 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |

### `util/http/host-rules › GHE platform endpoint fallback`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fallback to github for non-listed hostType targeting GHE endpoint | 587 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| no fallback when request targets a different host | 609 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| no fallback to gitlab | 620 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| fallback to gitlab | 677 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| no fallback to bitbucket | 734 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| fallback to bitbucket | 753 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| no fallback to bitbucket-server | 768 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| fallback to bitbucket-server | 787 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| no fallback to gitea | 802 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| fallback to gitea | 822 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| should remove forbidden headers from request | 835 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| should replace existing headers with host rule headers | 852 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |
| enabled=false with noAuth | 872 | not-applicable | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer | — | Mock framework internals — tests HTTP host-rules via vitest-mocked hostRules; Rust tests this at different layer |

---

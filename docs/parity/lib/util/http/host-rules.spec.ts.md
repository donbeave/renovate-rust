# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/host-rules.spec.ts
**Total tests:** 28 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/host-rules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds token | 63 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| adds token to an api.github.com URL | 78 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| adds auth | 95 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| adds custom auth | 108 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| skips | 126 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| uses http2 | 138 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| uses http keep-alive | 154 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| disables http2 | 166 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| noAuth | 183 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| certificateAuthority | 195 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| privateKey | 216 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| certificate | 237 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| no fallback to github | 258 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| fallback to github | 332 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| when multiple GitHub host types are set | 461 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |

### `util/http/host-rules › GHE platform endpoint fallback`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fallback to github for non-listed hostType targeting GHE endpoint | 587 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| no fallback when request targets a different host | 609 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| no fallback to gitlab | 620 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| fallback to gitlab | 677 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| no fallback to bitbucket | 734 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| fallback to bitbucket | 753 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| no fallback to bitbucket-server | 768 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| fallback to bitbucket-server | 787 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| no fallback to gitea | 802 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| fallback to gitea | 822 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| should remove forbidden headers from request | 835 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| should replace existing headers with host rule headers | 852 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |
| enabled=false with noAuth | 872 | not-applicable | — | — | tests applyHostRule/findMatchingRule for GotOptions; Rust HTTP uses different host-rules model |

---


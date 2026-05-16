# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/host-rules.spec.ts
**Total tests:** 28 | **Ported:** 0 | **Actionable:** 28 | **Status:** pending

### `util/http/host-rules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds token | 63 | pending | — | — | — |
| adds token to an api.github.com URL | 78 | pending | — | — | — |
| adds auth | 95 | pending | — | — | — |
| adds custom auth | 108 | pending | — | — | — |
| skips | 126 | pending | — | — | — |
| uses http2 | 138 | pending | — | — | — |
| uses http keep-alive | 154 | pending | — | — | — |
| disables http2 | 166 | pending | — | — | — |
| noAuth | 183 | pending | — | — | — |
| certificateAuthority | 195 | pending | — | — | — |
| privateKey | 216 | pending | — | — | — |
| certificate | 237 | pending | — | — | — |
| no fallback to github | 258 | pending | — | — | — |
| fallback to github | 332 | pending | — | — | — |
| when multiple GitHub host types are set | 461 | pending | — | — | — |

### `util/http/host-rules › GHE platform endpoint fallback`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fallback to github for non-listed hostType targeting GHE endpoint | 587 | pending | — | — | — |
| no fallback when request targets a different host | 609 | pending | — | — | — |
| no fallback to gitlab | 620 | pending | — | — | — |
| fallback to gitlab | 677 | pending | — | — | — |
| no fallback to bitbucket | 734 | pending | — | — | — |
| fallback to bitbucket | 753 | pending | — | — | — |
| no fallback to bitbucket-server | 768 | pending | — | — | — |
| fallback to bitbucket-server | 787 | pending | — | — | — |
| no fallback to gitea | 802 | pending | — | — | — |
| fallback to gitea | 822 | pending | — | — | — |
| should remove forbidden headers from request | 835 | pending | — | — | — |
| should replace existing headers with host rule headers | 852 | pending | — | — | — |
| enabled=false with noAuth | 872 | pending | — | — | — |

---


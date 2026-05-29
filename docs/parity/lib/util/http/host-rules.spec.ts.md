# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/host-rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/host-rules.spec.ts
**Total tests:** 28 | **Ported:** 0 | **Actionable:** 28 | **Status:** not-applicable

### `util/http/host-rules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds token | 63 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| adds token to an api.github.com URL | 78 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| adds auth | 95 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| adds custom auth | 108 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| skips | 126 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| uses http2 | 138 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| uses http keep-alive | 154 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| disables http2 | 166 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| noAuth | 183 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| certificateAuthority | 195 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| privateKey | 216 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| certificate | 237 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| no fallback to github | 258 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| fallback to github | 332 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| when multiple GitHub host types are set | 461 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|

### `util/http/host-rules › GHE platform endpoint fallback`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fallback to github for non-listed hostType targeting GHE endpoint | 587 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| no fallback when request targets a different host | 609 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| no fallback to gitlab | 620 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| fallback to gitlab | 677 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| no fallback to bitbucket | 734 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| fallback to bitbucket | 753 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| no fallback to bitbucket-server | 768 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| fallback to bitbucket-server | 787 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| no fallback to gitea | 802 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| fallback to gitea | 822 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| should remove forbidden headers from request | 835 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| should replace existing headers with host rule headers | 852 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|
| enabled=false with noAuth | 872 | not-applicable | — | — | mocking framework internals — vi.mock(global-agent) proxy module; TypeScript HTTP host-rule application pipeline|

---

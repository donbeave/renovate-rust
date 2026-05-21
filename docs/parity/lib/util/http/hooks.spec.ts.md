# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/hooks.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/hooks.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/hooks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns $expected for status code $statusCode and followRedirect $followRedirect | 5 | not-applicable | — | — | TypeScript `got` HTTP library hook workaround (got#1489); Rust uses reqwest which does not have this issue |

---


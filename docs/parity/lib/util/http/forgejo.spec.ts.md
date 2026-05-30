# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/forgejo.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/forgejo.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/http/forgejo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports responses without pagination when enabled | 15 | not-applicable | — | — | Forgejo pagination wrapper; Rust uses generic `reqwest` client without platform-specific pagination |
| supports root-level pagination | 27 | not-applicable | — | — | Forgejo pagination wrapper; Rust uses generic `reqwest` client without platform-specific pagination |
| supports pagination on data property | 46 | not-applicable | — | — | Forgejo pagination wrapper; Rust uses generic `reqwest` client without platform-specific pagination |
| handles pagination with empty response | 66 | not-applicable | — | — | Forgejo pagination wrapper; Rust uses generic `reqwest` client without platform-specific pagination |

---


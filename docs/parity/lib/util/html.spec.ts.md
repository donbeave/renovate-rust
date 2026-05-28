# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/html.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/html.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** partial

### `util/html`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses HTML | 5 | not-applicable | — | — | Requires node-html-parser (DOM API); no direct Rust equivalent with same API |
| returns empty | 14 | not-applicable | — | — | Requires node-html-parser (DOM API); no direct Rust equivalent with same API |
| parses HTML: PRE block hides child nodes | 19 | not-applicable | — | — | Requires node-html-parser (DOM API); no direct Rust equivalent with same API |
| parses HTML: use additional options to discover child nodes on PRE blocks | 25 | not-applicable | — | — | Requires node-html-parser (DOM API); no direct Rust equivalent with same API |

---


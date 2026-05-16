# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/html.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/html.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/html`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses HTML | 5 | not-applicable | — | — | Renovate's Node HTML parser wrapper is not implemented as a Rust API; Rust HTML manager uses extractor-specific parsing. |
| returns empty | 14 | not-applicable | — | — | Renovate's Node HTML parser wrapper is not implemented as a Rust API; Rust HTML manager uses extractor-specific parsing. |
| parses HTML: PRE block hides child nodes | 19 | not-applicable | — | — | Renovate's node-html-parser PRE-block option behavior has no Rust API equivalent. |
| parses HTML: use additional options to discover child nodes on PRE blocks | 25 | not-applicable | — | — | Renovate's node-html-parser option passthrough has no Rust API equivalent. |

---


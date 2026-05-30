# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/html.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/html.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `util/html`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses HTML | 5 | not-applicable | — | — | TS-library-specific; tests node-html-parser npm package DOM manipulation (HTMLElement.childNodes, querySelectorAll); Rust uses different HTML parsing|
| returns empty | 14 | not-applicable | — | — | TS-library-specific; tests node-html-parser npm package DOM manipulation (HTMLElement.childNodes, querySelectorAll); Rust uses different HTML parsing|
| parses HTML: PRE block hides child nodes | 19 | not-applicable | — | — | TS-library-specific; tests node-html-parser npm package DOM manipulation (HTMLElement.childNodes, querySelectorAll); Rust uses different HTML parsing|
| parses HTML: use additional options to discover child nodes on PRE blocks | 25 | not-applicable | — | — | TS-library-specific; tests node-html-parser npm package DOM manipulation (HTMLElement.childNodes, querySelectorAll); Rust uses different HTML parsing|

---


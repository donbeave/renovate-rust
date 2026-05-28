# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/markdown.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/markdown.spec.ts
**Total tests:** 3 | **Ported:** 1 | **Actionable:** 0 | **Status:** partial

### `util/markdown › .linkify`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 33 | not-applicable | — | — | remark-linkify-regex npm library for GitHub Markdown link generation; not ported to Rust |
| works with gitlab | 38 | not-applicable | — | — | remark-linkify-regex npm library for GitHub Markdown link generation; not ported to Rust |
| sanitizeMarkdown check massaged release notes | 48 | ported | `util.rs` | `test_sanitize_markdown` | — |

---


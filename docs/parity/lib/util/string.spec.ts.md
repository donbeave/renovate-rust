# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/string.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/string.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/string › replaceAt`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaceAt inserts newString which is one char longer than oldString | 11 | not-applicable | — | — | Renovate's TypeScript string replacement helper is not implemented as a shared Rust API. |
| replaceAt inserts newString which is significantly longer than oldString | 22 | not-applicable | — | — | Renovate's TypeScript string replacement helper is not implemented as a shared Rust API. |

### `util/string › looseEquals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reverts to literal match if either is falsey | 35 | not-applicable | — | — | Renovate's JavaScript truthiness-aware loose equality helper has no Rust API equivalent. |

### `util/string`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| coerceString | 42 | not-applicable | — | — | Renovate's TypeScript nullable string coercion helper is not implemented as a shared Rust API. |

### `util/string › stripTemplates`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| "$input" -> "$expected" | 51 | not-applicable | — | — | Renovate's generic Handlebars/Jinja template stripping helper is not implemented as a shared Rust API; Rust template handling is extractor-specific. |

### `util/string › capitalize`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| capitalizes | 81 | not-applicable | — | — | Renovate's TypeScript capitalization helper is not implemented as a shared Rust API. |

---


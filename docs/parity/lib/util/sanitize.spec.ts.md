# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/sanitize.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/sanitize.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/sanitize`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitizes empty string | 15 | not-applicable | — | — | Renovate's TypeScript global secret sanitizer registry is not implemented as a Rust API. |
| sanitizes secrets from strings | 21 | not-applicable | — | — | Renovate's TypeScript global/repo secret sanitizer registry and log redaction helper are not implemented as a Rust API. |
| sanitizes github app tokens | 40 | not-applicable | — | — | Renovate's TypeScript GitHub App token redaction helper is not implemented as a Rust API. |

---


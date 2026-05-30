# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/promises.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/promises.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** done

### `util/promises › all`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 6 | not-applicable | — | — |Tests TypeScript-specific p-all/p-map promise wrappers and ExternalHostError aggregation; Rust uses native Future/join_all |

### `util/promises › map`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 17 | not-applicable | — | — |Tests TypeScript-specific p-all/p-map promise wrappers and ExternalHostError aggregation; Rust uses native Future/join_all |

### `util/promises › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws first ExternalHostError found | 24 | not-applicable | — | — |Tests TypeScript-specific p-all/p-map promise wrappers and ExternalHostError aggregation; Rust uses native Future/join_all |
| throws first error if error messages are all the same | 43 | not-applicable | — | — |Tests TypeScript-specific p-all/p-map promise wrappers and ExternalHostError aggregation; Rust uses native Future/join_all |
| throws aggregate error for different error messages | 62 | not-applicable | — | — |Tests TypeScript-specific p-all/p-map promise wrappers and ExternalHostError aggregation; Rust uses native Future/join_all |
| re-throws when stopOnError=true | 69 | not-applicable | — | — |Tests TypeScript-specific p-all/p-map promise wrappers and ExternalHostError aggregation; Rust uses native Future/join_all |

---


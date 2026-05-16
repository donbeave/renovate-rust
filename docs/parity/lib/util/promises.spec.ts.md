# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/promises.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/promises.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/promises › all`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 6 | not-applicable | — | — | Renovate's TypeScript promise queue helper is not implemented as a Rust API; Rust uses Tokio futures directly in call sites. |

### `util/promises › map`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 17 | not-applicable | — | — | Renovate's TypeScript promise map helper is not implemented as a Rust API; Rust uses Tokio futures directly in call sites. |

### `util/promises › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws first ExternalHostError found | 24 | not-applicable | — | — | Renovate's TypeScript promise helper ExternalHostError aggregation policy is not implemented as a Rust API. |
| throws first error if error messages are all the same | 43 | not-applicable | — | — | Renovate's TypeScript promise helper error aggregation policy is not implemented as a Rust API. |
| throws aggregate error for different error messages | 62 | not-applicable | — | — | Renovate's TypeScript AggregateError behavior has no shared Rust API equivalent. |
| re-throws when stopOnError=true | 69 | not-applicable | — | — | Renovate's TypeScript promise helper stopOnError policy is not implemented as a Rust API. |

---


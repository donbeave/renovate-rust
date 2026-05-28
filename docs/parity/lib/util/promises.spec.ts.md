# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/promises.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/promises.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/promises › all`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 6 | not-applicable | — | — | JavaScript async promise concurrency; Rust uses async/await with tokio natively - no direct library equivalent needed |

### `util/promises › map`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 17 | not-applicable | — | — | JavaScript async promise concurrency; Rust uses async/await with tokio natively - no direct library equivalent needed |

### `util/promises › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws first ExternalHostError found | 24 | not-applicable | — | — | JavaScript async promise concurrency; Rust uses async/await with tokio natively - no direct library equivalent needed |
| throws first error if error messages are all the same | 43 | not-applicable | — | — | JavaScript async promise concurrency; Rust uses async/await with tokio natively - no direct library equivalent needed |
| throws aggregate error for different error messages | 62 | not-applicable | — | — | JavaScript async promise concurrency; Rust uses async/await with tokio natively - no direct library equivalent needed |
| re-throws when stopOnError=true | 69 | not-applicable | — | — | JavaScript async promise concurrency; Rust uses async/await with tokio natively - no direct library equivalent needed |

---


# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/jsonata.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/jsonata.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/jsonata › getExpression`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an expression | 6 | not-applicable | — | — | Renovate's TypeScript JSONata expression engine wrapper is not implemented as a Rust API; Rust only validates JSONata config syntax. |
| should return an error | 10 | not-applicable | — | — | Renovate's TypeScript JSONata expression engine wrapper is not implemented as a Rust API; Rust only validates JSONata config syntax. |

### `util/jsonata › getExpression › $detectPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return platform for known URL | 15 | not-applicable | — | — | Renovate's TypeScript JSONata custom `$detectPlatform` evaluator is not implemented as a Rust API. |
| should return null for unknown URL | 28 | not-applicable | — | — | Renovate's TypeScript JSONata custom `$detectPlatform` evaluator is not implemented as a Rust API. |

### `util/jsonata › getExpression › concurrent evaluation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should maintain data isolation when evaluating same expression concurrently | 47 | not-applicable | — | — | Renovate's TypeScript JSONata runtime and concurrent evaluation behavior are not implemented as a Rust API. |
| should maintain data isolation with complex $$ references | 73 | not-applicable | — | — | Renovate's TypeScript JSONata runtime and concurrent evaluation behavior are not implemented as a Rust API. |

---


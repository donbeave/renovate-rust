# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/jsonata.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/jsonata.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `util/jsonata › getExpression`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an expression | 6 | not-applicable | — | — | TS-library-specific; tests jsonata npm package expression evaluation and $detectPlatform custom function; Rust would use different expression evaluation|
| should return an error | 10 | not-applicable | — | — | TS-library-specific; tests jsonata npm package expression evaluation and $detectPlatform custom function; Rust would use different expression evaluation|

### `util/jsonata › getExpression › $detectPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return platform for known URL | 15 | not-applicable | — | — | TS-library-specific; tests jsonata npm package expression evaluation and $detectPlatform custom function; Rust would use different expression evaluation|
| should return null for unknown URL | 28 | not-applicable | — | — | TS-library-specific; tests jsonata npm package expression evaluation and $detectPlatform custom function; Rust would use different expression evaluation|

### `util/jsonata › getExpression › concurrent evaluation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should maintain data isolation when evaluating same expression concurrently | 47 | not-applicable | — | — | TS-library-specific; tests jsonata npm package expression evaluation and $detectPlatform custom function; Rust would use different expression evaluation|
| should maintain data isolation with complex $$ references | 73 | not-applicable | — | — | TS-library-specific; tests jsonata npm package expression evaluation and $detectPlatform custom function; Rust would use different expression evaluation|

---


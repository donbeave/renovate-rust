# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/template/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/template/index.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 54 | **Status:** done

### `util/template/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if cannot compile | 17 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| has valid exposed config options | 21 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| filters out disallowed fields | 29 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| containsString | 43 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| unless with equals - 1 | 51 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| unless with equals - 2 | 64 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| not containsString | 75 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| and returns true when all parameters are true | 83 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| and returns false when at least one parameter is false | 91 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| or returns true when at least one is true | 99 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| or returns false when all are false | 107 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| string to pretty JSON | 115 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| to JSON | 122 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| to JSON empty array | 137 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| to JSON empty object | 143 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| to Object passing illegal number of elements | 149 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| build complex json | 155 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| do not escape common range symbols: $input -> $output | 174 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| lowercase | 191 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| has access to basic environment variables (basicEnvVars) | 197 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| and has access to custom variables (customEnvVariables) | 203 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| and has access to prBodyDefinitions | 209 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| replace | 225 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| add | 234 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| add - throws if inputs are invalid | 240 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › proxyCompileInput`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| accessing allowed fields | 261 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| supports object nesting | 272 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| supports array nesting | 288 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › percent encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 308 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| decodes values | 316 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › base64 encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 326 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles null values gracefully | 334 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles undefined values gracefully | 341 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › base64 decoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decode values | 350 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles null values gracefully | 358 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles undefined values gracefully | 365 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › equals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals | 374 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| not equals | 385 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| not strict equals | 396 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › includes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| includes is true | 408 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| includes is false | 419 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| includes with incorrect type first argument | 430 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| includes with incorrect type second argument | 441 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › split`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array on non string input | 454 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| should return empty array on missing parameter | 461 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| should return array on success | 468 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| should return array element | 475 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › lookupArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| performs lookup for every array element | 487 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles null input array | 512 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles empty string key | 524 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles null key | 540 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

### `util/template/index › distinct`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips duplicate values | 558 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles null elements | 585 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |
| handles null input | 597 | not-applicable | — | — | Requires vi.mock(exec/utils) mock infrastructure |

---


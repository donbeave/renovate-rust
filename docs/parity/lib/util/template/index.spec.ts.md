# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/template/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/template/index.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/template/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if cannot compile | 17 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| has valid exposed config options | 21 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| filters out disallowed fields | 29 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| containsString | 43 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| unless with equals - 1 | 51 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| unless with equals - 2 | 64 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| not containsString | 75 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| and returns true when all parameters are true | 83 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| and returns false when at least one parameter is false | 91 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| or returns true when at least one is true | 99 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| or returns false when all are false | 107 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| string to pretty JSON | 115 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| to JSON | 122 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| to JSON empty array | 137 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| to JSON empty object | 143 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| to Object passing illegal number of elements | 149 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| build complex json | 155 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| do not escape common range symbols: $input -> $output | 174 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| lowercase | 191 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| has access to basic environment variables (basicEnvVars) | 197 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| and has access to custom variables (customEnvVariables) | 203 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| and has access to prBodyDefinitions | 209 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| replace | 225 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| add | 234 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| add - throws if inputs are invalid | 240 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › proxyCompileInput`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| accessing allowed fields | 261 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| supports object nesting | 272 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| supports array nesting | 288 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › percent encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 308 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| decodes values | 316 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › base64 encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 326 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles null values gracefully | 334 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles undefined values gracefully | 341 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › base64 decoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decode values | 350 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles null values gracefully | 358 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles undefined values gracefully | 365 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › equals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals | 374 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| not equals | 385 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| not strict equals | 396 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › includes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| includes is true | 408 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| includes is false | 419 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| includes with incorrect type first argument | 430 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| includes with incorrect type second argument | 441 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › split`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array on non string input | 454 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| should return empty array on missing parameter | 461 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| should return array on success | 468 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| should return array element | 475 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › lookupArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| performs lookup for every array element | 487 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles null input array | 512 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles empty string key | 524 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles null key | 540 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

### `util/template/index › distinct`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips duplicate values | 558 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles null elements | 585 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |
| handles null input | 597 | not-applicable | — | — | tests Handlebars template rendering with TypeScript-specific helpers; Rust would use own template engine |

---


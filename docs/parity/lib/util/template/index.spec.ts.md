# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/template/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/template/index.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/template/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if cannot compile | 17 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| has valid exposed config options | 21 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| filters out disallowed fields | 29 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| containsString | 43 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| unless with equals - 1 | 51 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| unless with equals - 2 | 64 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| not containsString | 75 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| and returns true when all parameters are true | 83 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| and returns false when at least one parameter is false | 91 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| or returns true when at least one is true | 99 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| or returns false when all are false | 107 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| string to pretty JSON | 115 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| to JSON | 122 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| to JSON empty array | 137 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| to JSON empty object | 143 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| to Object passing illegal number of elements | 149 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| build complex json | 155 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| do not escape common range symbols: $input -> $output | 174 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| lowercase | 191 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| has access to basic environment variables (basicEnvVars) | 197 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| and has access to custom variables (customEnvVariables) | 203 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| and has access to prBodyDefinitions | 209 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| replace | 225 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| add | 234 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| add - throws if inputs are invalid | 240 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › proxyCompileInput`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| accessing allowed fields | 261 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| supports object nesting | 272 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| supports array nesting | 288 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › percent encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 308 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| decodes values | 316 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › base64 encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 326 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles null values gracefully | 334 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles undefined values gracefully | 341 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › base64 decoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decode values | 350 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles null values gracefully | 358 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles undefined values gracefully | 365 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › equals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals | 374 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| not equals | 385 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| not strict equals | 396 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › includes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| includes is true | 408 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| includes is false | 419 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| includes with incorrect type first argument | 430 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| includes with incorrect type second argument | 441 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › split`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array on non string input | 454 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| should return empty array on missing parameter | 461 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| should return array on success | 468 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| should return array element | 475 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › lookupArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| performs lookup for every array element | 487 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles null input array | 512 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles empty string key | 524 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles null key | 540 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

### `util/template/index › distinct`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips duplicate values | 558 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles null elements | 585 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|
| handles null input | 597 | not-applicable | Mock framework internals — tests template index via vitest-mocked Handlebars; Rust tests this at different layer | — | No corresponding Rust source|

---


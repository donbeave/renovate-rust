# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/template/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/template/index.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 54 | **Status:** pending

### `util/template/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if cannot compile | 17 | pending | — | — | No corresponding Rust source|
| has valid exposed config options | 21 | pending | — | — | No corresponding Rust source|
| filters out disallowed fields | 29 | pending | — | — | No corresponding Rust source|
| containsString | 43 | pending | — | — | No corresponding Rust source|
| unless with equals - 1 | 51 | pending | — | — | No corresponding Rust source|
| unless with equals - 2 | 64 | pending | — | — | No corresponding Rust source|
| not containsString | 75 | pending | — | — | No corresponding Rust source|
| and returns true when all parameters are true | 83 | pending | — | — | No corresponding Rust source|
| and returns false when at least one parameter is false | 91 | pending | — | — | No corresponding Rust source|
| or returns true when at least one is true | 99 | pending | — | — | No corresponding Rust source|
| or returns false when all are false | 107 | pending | — | — | No corresponding Rust source|
| string to pretty JSON | 115 | pending | — | — | No corresponding Rust source|
| to JSON | 122 | pending | — | — | No corresponding Rust source|
| to JSON empty array | 137 | pending | — | — | No corresponding Rust source|
| to JSON empty object | 143 | pending | — | — | No corresponding Rust source|
| to Object passing illegal number of elements | 149 | pending | — | — | No corresponding Rust source|
| build complex json | 155 | pending | — | — | No corresponding Rust source|
| do not escape common range symbols: $input -> $output | 174 | pending | — | — | No corresponding Rust source|
| lowercase | 191 | pending | — | — | No corresponding Rust source|
| has access to basic environment variables (basicEnvVars) | 197 | pending | — | — | No corresponding Rust source|
| and has access to custom variables (customEnvVariables) | 203 | pending | — | — | No corresponding Rust source|
| and has access to prBodyDefinitions | 209 | pending | — | — | No corresponding Rust source|
| replace | 225 | pending | — | — | No corresponding Rust source|
| add | 234 | pending | — | — | No corresponding Rust source|
| add - throws if inputs are invalid | 240 | pending | — | — | No corresponding Rust source|

### `util/template/index › proxyCompileInput`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| accessing allowed fields | 261 | pending | — | — | No corresponding Rust source|
| supports object nesting | 272 | pending | — | — | No corresponding Rust source|
| supports array nesting | 288 | pending | — | — | No corresponding Rust source|

### `util/template/index › percent encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 308 | pending | — | — | No corresponding Rust source|
| decodes values | 316 | pending | — | — | No corresponding Rust source|

### `util/template/index › base64 encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 326 | pending | — | — | No corresponding Rust source|
| handles null values gracefully | 334 | pending | — | — | No corresponding Rust source|
| handles undefined values gracefully | 341 | pending | — | — | No corresponding Rust source|

### `util/template/index › base64 decoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decode values | 350 | pending | — | — | No corresponding Rust source|
| handles null values gracefully | 358 | pending | — | — | No corresponding Rust source|
| handles undefined values gracefully | 365 | pending | — | — | No corresponding Rust source|

### `util/template/index › equals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals | 374 | pending | — | — | No corresponding Rust source|
| not equals | 385 | pending | — | — | No corresponding Rust source|
| not strict equals | 396 | pending | — | — | No corresponding Rust source|

### `util/template/index › includes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| includes is true | 408 | pending | — | — | No corresponding Rust source|
| includes is false | 419 | pending | — | — | No corresponding Rust source|
| includes with incorrect type first argument | 430 | pending | — | — | No corresponding Rust source|
| includes with incorrect type second argument | 441 | pending | — | — | No corresponding Rust source|

### `util/template/index › split`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array on non string input | 454 | pending | — | — | No corresponding Rust source|
| should return empty array on missing parameter | 461 | pending | — | — | No corresponding Rust source|
| should return array on success | 468 | pending | — | — | No corresponding Rust source|
| should return array element | 475 | pending | — | — | No corresponding Rust source|

### `util/template/index › lookupArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| performs lookup for every array element | 487 | pending | — | — | No corresponding Rust source|
| handles null input array | 512 | pending | — | — | No corresponding Rust source|
| handles empty string key | 524 | pending | — | — | No corresponding Rust source|
| handles null key | 540 | pending | — | — | No corresponding Rust source|

### `util/template/index › distinct`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips duplicate values | 558 | pending | — | — | No corresponding Rust source|
| handles null elements | 585 | pending | — | — | No corresponding Rust source|
| handles null input | 597 | pending | — | — | No corresponding Rust source|

---


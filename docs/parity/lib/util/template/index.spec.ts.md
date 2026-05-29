# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/template/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/template/index.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 54 | **Status:** pending

### `util/template/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if cannot compile | 17 | pending | — | — | —|
| has valid exposed config options | 21 | pending | — | — | —|
| filters out disallowed fields | 29 | pending | — | — | —|
| containsString | 43 | pending | — | — | —|
| unless with equals - 1 | 51 | pending | — | — | —|
| unless with equals - 2 | 64 | pending | — | — | —|
| not containsString | 75 | pending | — | — | —|
| and returns true when all parameters are true | 83 | pending | — | — | —|
| and returns false when at least one parameter is false | 91 | pending | — | — | —|
| or returns true when at least one is true | 99 | pending | — | — | —|
| or returns false when all are false | 107 | pending | — | — | —|
| string to pretty JSON | 115 | pending | — | — | —|
| to JSON | 122 | pending | — | — | —|
| to JSON empty array | 137 | pending | — | — | —|
| to JSON empty object | 143 | pending | — | — | —|
| to Object passing illegal number of elements | 149 | pending | — | — | —|
| build complex json | 155 | pending | — | — | —|
| do not escape common range symbols: $input -> $output | 174 | pending | — | — | —|
| lowercase | 191 | pending | — | — | —|
| has access to basic environment variables (basicEnvVars) | 197 | pending | — | — | —|
| and has access to custom variables (customEnvVariables) | 203 | pending | — | — | —|
| and has access to prBodyDefinitions | 209 | pending | — | — | —|
| replace | 225 | pending | — | — | —|
| add | 234 | pending | — | — | —|
| add - throws if inputs are invalid | 240 | pending | — | — | —|

### `util/template/index › proxyCompileInput`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| accessing allowed fields | 261 | pending | — | — | —|
| supports object nesting | 272 | pending | — | — | —|
| supports array nesting | 288 | pending | — | — | —|

### `util/template/index › percent encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 308 | pending | — | — | —|
| decodes values | 316 | pending | — | — | —|

### `util/template/index › base64 encoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| encodes values | 326 | pending | — | — | —|
| handles null values gracefully | 334 | pending | — | — | —|
| handles undefined values gracefully | 341 | pending | — | — | —|

### `util/template/index › base64 decoding`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| decode values | 350 | pending | — | — | —|
| handles null values gracefully | 358 | pending | — | — | —|
| handles undefined values gracefully | 365 | pending | — | — | —|

### `util/template/index › equals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals | 374 | pending | — | — | —|
| not equals | 385 | pending | — | — | —|
| not strict equals | 396 | pending | — | — | —|

### `util/template/index › includes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| includes is true | 408 | pending | — | — | —|
| includes is false | 419 | pending | — | — | —|
| includes with incorrect type first argument | 430 | pending | — | — | —|
| includes with incorrect type second argument | 441 | pending | — | — | —|

### `util/template/index › split`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array on non string input | 454 | pending | — | — | —|
| should return empty array on missing parameter | 461 | pending | — | — | —|
| should return array on success | 468 | pending | — | — | —|
| should return array element | 475 | pending | — | — | —|

### `util/template/index › lookupArray`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| performs lookup for every array element | 487 | pending | — | — | —|
| handles null input array | 512 | pending | — | — | —|
| handles empty string key | 524 | pending | — | — | —|
| handles null key | 540 | pending | — | — | —|

### `util/template/index › distinct`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips duplicate values | 558 | pending | — | — | —|
| handles null elements | 585 | pending | — | — | —|
| handles null input | 597 | pending | — | — | —|

---


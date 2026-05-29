# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/npmrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/npmrc.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** not-applicable

### `modules/datasource/npm/npmrc › getMatchHostFromNpmrcHost()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses //host | 22 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| parses //host/path | 28 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| parses https://host | 34 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|

### `modules/datasource/npm/npmrc › convertNpmrcToRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid registries | 42 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| handles naked auth | 50 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| handles host, path and auth | 66 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| handles host, path, port and auth | 84 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| handles naked authToken | 103 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| handles host authToken | 118 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| handles username and _password | 151 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|

### `modules/datasource/npm/npmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitize _auth | 174 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| sanitize _authtoken | 181 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| sanitize _password | 191 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| sanitize _authtoken with high trust | 203 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|
| ignores localhost | 214 | not-applicable | — | — | mocking framework internals — vi.mock on sanitize; TypeScript npm registry config parsing|

---

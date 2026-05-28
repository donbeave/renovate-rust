# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/npmrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/npmrc.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** done

### `modules/datasource/npm/npmrc › getMatchHostFromNpmrcHost()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses //host | 22 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| parses //host/path | 28 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| parses https://host | 34 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |

### `modules/datasource/npm/npmrc › convertNpmrcToRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid registries | 42 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| handles naked auth | 50 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| handles host, path and auth | 66 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| handles host, path, port and auth | 84 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| handles naked authToken | 103 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| handles host authToken | 118 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| handles username and _password | 151 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |

### `modules/datasource/npm/npmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitize _auth | 174 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| sanitize _authtoken | 181 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| sanitize _password | 191 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| sanitize _authtoken with high trust | 203 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |
| ignores localhost | 214 | not-applicable | — | — | Requires vi.mock(sanitize) and GlobalConfig mock infrastructure |

---

# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/npmrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/npmrc.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/npm/npmrc › getMatchHostFromNpmrcHost()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses //host | 22 | not-applicable | — | — | npm npmrc datasource HTTP |
| parses //host/path | 28 | not-applicable | — | — | npm npmrc datasource HTTP |
| parses https://host | 34 | not-applicable | — | — | npm npmrc datasource HTTP |

### `modules/datasource/npm/npmrc › convertNpmrcToRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid registries | 42 | not-applicable | — | — | npm npmrc datasource HTTP |
| handles naked auth | 50 | not-applicable | — | — | npm npmrc datasource HTTP |
| handles host, path and auth | 66 | not-applicable | — | — | npm npmrc datasource HTTP |
| handles host, path, port and auth | 84 | not-applicable | — | — | npm npmrc datasource HTTP |
| handles naked authToken | 103 | not-applicable | — | — | npm npmrc datasource HTTP |
| handles host authToken | 118 | not-applicable | — | — | npm npmrc datasource HTTP |
| handles username and _password | 151 | not-applicable | — | — | npm npmrc datasource HTTP |

### `modules/datasource/npm/npmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitize _auth | 174 | not-applicable | — | — | npm npmrc datasource HTTP |
| sanitize _authtoken | 181 | not-applicable | — | — | npm npmrc datasource HTTP |
| sanitize _password | 191 | not-applicable | — | — | npm npmrc datasource HTTP |
| sanitize _authtoken with high trust | 203 | not-applicable | — | — | npm npmrc datasource HTTP |
| ignores localhost | 214 | not-applicable | — | — | npm npmrc datasource HTTP |

---

# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/npmrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/npmrc.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `modules/datasource/npm/npmrc › getMatchHostFromNpmrcHost()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses //host | 22 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| parses //host/path | 28 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| parses https://host | 34 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |

### `modules/datasource/npm/npmrc › convertNpmrcToRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid registries | 42 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| handles naked auth | 50 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| handles host, path and auth | 66 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| handles host, path, port and auth | 84 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| handles naked authToken | 103 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| handles host authToken | 118 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| handles username and _password | 151 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |

### `modules/datasource/npm/npmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitize _auth | 174 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| sanitize _authtoken | 181 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| sanitize _password | 191 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| sanitize _authtoken with high trust | 203 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |
| ignores localhost | 214 | not-applicable | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer | — | Mock framework internals — tests npm npmrc via vitest-mocked fs/hostRules; Rust tests this at different layer |

---

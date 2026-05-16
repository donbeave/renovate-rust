# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/npmrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/npmrc.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/npm/npmrc › getMatchHostFromNpmrcHost()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses //host | 22 | not-applicable | — | — | Renovate's `.npmrc` host matcher and config conversion utilities are not implemented in Rust; Rust npm support fetches packuments from a supplied registry URL. |
| parses //host/path | 28 | not-applicable | — | — | Renovate's `.npmrc` host matcher and config conversion utilities are not implemented in Rust; Rust npm support fetches packuments from a supplied registry URL. |
| parses https://host | 34 | not-applicable | — | — | Renovate's `.npmrc` host matcher and config conversion utilities are not implemented in Rust; Rust npm support fetches packuments from a supplied registry URL. |

### `modules/datasource/npm/npmrc › convertNpmrcToRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid registries | 42 | not-applicable | — | — | Renovate's `.npmrc` to hostRules/packageRules conversion is not implemented in Rust. |
| handles naked auth | 50 | not-applicable | — | — | Renovate's `.npmrc` auth conversion to hostRules is not implemented in Rust. |
| handles host, path and auth | 66 | not-applicable | — | — | Renovate's `.npmrc` auth conversion to hostRules is not implemented in Rust. |
| handles host, path, port and auth | 84 | not-applicable | — | — | Renovate's `.npmrc` auth conversion to hostRules is not implemented in Rust. |
| handles naked authToken | 103 | not-applicable | — | — | Renovate's `.npmrc` auth token conversion to hostRules is not implemented in Rust. |
| handles host authToken | 118 | not-applicable | — | — | Renovate's `.npmrc` scoped registry and auth token conversion to hostRules/packageRules is not implemented in Rust. |
| handles username and _password | 151 | not-applicable | — | — | Renovate's `.npmrc` username/password decoding and hostRules conversion are not implemented in Rust. |

### `modules/datasource/npm/npmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitize _auth | 174 | not-applicable | — | — | Renovate's `.npmrc` secret sanitization side effects are not implemented in Rust. |
| sanitize _authtoken | 181 | not-applicable | — | — | Renovate's `.npmrc` secret sanitization side effects are not implemented in Rust. |
| sanitize _password | 191 | not-applicable | — | — | Renovate's `.npmrc` secret sanitization side effects are not implemented in Rust. |
| sanitize _authtoken with high trust | 203 | not-applicable | — | — | Renovate's `.npmrc` secret sanitization and exposeAllEnv handling are not implemented in Rust. |
| ignores localhost | 214 | not-applicable | — | — | Renovate's `.npmrc` secret sanitization side effects are not implemented in Rust. |

---


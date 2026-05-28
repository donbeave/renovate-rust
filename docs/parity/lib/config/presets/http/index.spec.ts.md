# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/http/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/http/index.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `config/presets/http/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return parsed JSON | 13 | not-applicable | — | — | Uses httpMock (nock) + hostRules; HTTP mock infrastructure not portable |
| should return parsed JSON5 | 19 | not-applicable | — | — | Uses httpMock (nock) + hostRules; HTTP mock infrastructure not portable |
| throws if fails to parse | 30 | not-applicable | — | — | Uses httpMock (nock) + hostRules; HTTP mock infrastructure not portable |
| throws if file not found | 38 | not-applicable | — | — | Uses httpMock (nock) + hostRules; HTTP mock infrastructure not portable |
| throws on malformed URL | 46 | not-applicable | — | — | Uses httpMock (nock) + hostRules; HTTP mock infrastructure not portable |
| throws external host error | 51 | not-applicable | — | — | Uses httpMock (nock) + hostRules; HTTP mock infrastructure not portable |

---


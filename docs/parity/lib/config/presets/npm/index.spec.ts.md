# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/npm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/npm/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** not-applicable

### `config/presets/npm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no package | 10 | not-applicable | — | — | Uses httpMock.scope to mock npm registry; HTTP mocking infrastructure not portable |
| should throw if no renovate-config | 17 | not-applicable | — | — | Uses httpMock.scope; HTTP mocking not portable |
| should throw if preset name not found | 48 | not-applicable | — | — | Uses httpMock.scope; HTTP mocking not portable |
| should return preset | 83 | not-applicable | — | — | Uses httpMock.scope; HTTP mocking not portable |

---

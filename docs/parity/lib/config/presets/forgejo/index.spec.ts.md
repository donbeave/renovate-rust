# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/forgejo/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** not-applicable

### `config/presets/forgejo/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 19 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| returns JSON5 | 36 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| throws external host error | 53 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |

### `config/presets/forgejo/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 73 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| throws if invalid content | 84 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| throws if fails to parse | 95 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should return default.json | 108 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query preset within the file | 120 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query subpreset | 134 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should return custom.json | 151 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query custom paths | 165 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should throws not-found | 180 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |

### `config/presets/forgejo/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 197 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| uses custom endpoint | 209 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| uses default endpoint with a tag | 228 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| uses custom endpoint with a tag | 246 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |

---


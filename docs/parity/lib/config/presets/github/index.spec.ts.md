# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/github/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 17 | **Status:** not-applicable

### `config/presets/github/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 17 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| throws external host error | 34 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |

### `config/presets/github/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 54 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| throws if invalid content | 65 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| throws if fails to parse | 76 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should return default.json | 89 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query preset within the file | 101 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query preset within the file when .json extension provided | 115 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query preset within the file when .json5 extension provided | 129 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query subpreset | 143 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should return custom.json | 160 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should query custom paths | 174 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| should throws not-found | 189 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |

### `config/presets/github/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 206 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| uses custom endpoint | 218 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| uses default endpoint with a tag | 238 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |
| uses custom endpoint with a tag | 256 | not-applicable | — | — | Uses httpMock + hostRules; HTTP mock infrastructure not portable to Rust |

---


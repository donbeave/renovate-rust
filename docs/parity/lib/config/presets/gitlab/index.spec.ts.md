# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/presets/gitlab/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/gitlab/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** not-applicable

### `config/presets/gitlab/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws EXTERNAL_HOST_ERROR | 12 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| throws if project could not be found | 22 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| throws if missing | 32 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| should return the preset | 47 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| should return the preset with a tag | 61 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| should query custom paths | 74 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| should query custom paths with .json extension | 92 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| should query custom paths with .json5 extension | 110 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |

### `config/presets/gitlab/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 130 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| uses custom endpoint | 148 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| uses default endpoint with a tag | 167 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |
| uses custom endpoint with a tag | 183 | not-applicable | — | — | Uses httpMock; HTTP mock infrastructure not portable to Rust |

---


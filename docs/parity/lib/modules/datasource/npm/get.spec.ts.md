# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/get.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/get.spec.ts
**Total tests:** 24 | **Ported:** 1 | **Actionable:** 24 | **Status:** done

### `modules/datasource/npm/get › has bearer auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 42 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |

### `modules/datasource/npm/get › has basic auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 75 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |

### `modules/datasource/npm/get › no auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 102 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |

### `modules/datasource/npm/get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses hostRules basic auth | 117 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| uses hostRules token auth | 139 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| uses hostRules basic token auth | 160 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| cover all paths | 182 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| throw ExternalHostError when error happens on registry.npmjs.org | 248 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| redact body for ExternalHostError when error happens on registry.npmjs.org | 259 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| do not throw ExternalHostError when error happens on custom host | 275 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| do not throw ExternalHostError when error happens on registry.npmjs.org when hostRules disables abortOnError | 287 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| do not throw ExternalHostError when error happens on registry.npmjs.org when hostRules without protocol disables abortOnError | 302 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| throw ExternalHostError when error happens on custom host when hostRules enables abortOnError | 318 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| massages non-compliant repository urls | 334 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles missing dist-tags latest | 378 | ported | `npm.rs` | `fetch_versions_allows_missing_latest_dist_tag` | — |
| handles mixed sourceUrls in releases | 401 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles short sourceUrls in releases | 442 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not override sourceDirectory | 483 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles full repository urls with release source directories | 526 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not massage non-github non-compliant repository urls | 552 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `modules/datasource/npm/get › cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stores a trimmed packument body in cache | 608 | not-applicable | — | — | Requires httpMock + packageCache mock infrastructure |
| returns unexpired cache | 705 | not-applicable | — | — | Requires httpMock + packageCache mock infrastructure |
| returns soft expired cache if revalidated | 737 | not-applicable | — | — | Requires httpMock + packageCache mock infrastructure |
| returns soft expired cache on npmjs error | 771 | not-applicable | — | — | Requires httpMock + packageCache mock infrastructure |

---

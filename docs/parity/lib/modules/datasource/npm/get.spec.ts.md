# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/get.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/get.spec.ts
**Total tests:** 24 | **Ported:** 1 | **Actionable:** 24 | **Status:** partial

### `modules/datasource/npm/get › has bearer auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 42 | pending | — | — | — |

### `modules/datasource/npm/get › has basic auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 75 | pending | — | — | — |

### `modules/datasource/npm/get › no auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 102 | pending | — | — | — |

### `modules/datasource/npm/get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses hostRules basic auth | 117 | pending | — | — | — |
| uses hostRules token auth | 139 | pending | — | — | — |
| uses hostRules basic token auth | 160 | pending | — | — | — |
| cover all paths | 182 | pending | — | — | — |
| throw ExternalHostError when error happens on registry.npmjs.org | 248 | pending | — | — | — |
| redact body for ExternalHostError when error happens on registry.npmjs.org | 259 | pending | — | — | — |
| do not throw ExternalHostError when error happens on custom host | 275 | pending | — | — | — |
| do not throw ExternalHostError when error happens on registry.npmjs.org when hostRules disables abortOnError | 287 | pending | — | — | — |
| do not throw ExternalHostError when error happens on registry.npmjs.org when hostRules without protocol disables abortOnError | 302 | pending | — | — | — |
| throw ExternalHostError when error happens on custom host when hostRules enables abortOnError | 318 | pending | — | — | — |
| massages non-compliant repository urls | 334 | pending | — | — | — |
| handles missing dist-tags latest | 378 | ported | `npm.rs` | `fetch_versions_allows_missing_latest_dist_tag` | — |
| handles mixed sourceUrls in releases | 401 | pending | — | — | — |
| handles short sourceUrls in releases | 442 | pending | — | — | — |
| does not override sourceDirectory | 483 | pending | — | — | — |
| handles full repository urls with release source directories | 526 | pending | — | — | — |
| does not massage non-github non-compliant repository urls | 552 | pending | — | — | — |

### `modules/datasource/npm/get › cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stores a trimmed packument body in cache | 608 | pending | — | — | — |
| returns unexpired cache | 705 | pending | — | — | — |
| returns soft expired cache if revalidated | 737 | pending | — | — | — |
| returns soft expired cache on npmjs error | 771 | pending | — | — | — |

---


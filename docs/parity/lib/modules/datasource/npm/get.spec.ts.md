# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/get.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/get.spec.ts
**Total tests:** 24 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/npm/get › has bearer auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 42 | not-applicable | — | — | Renovate's npmrc bearer auth resolution and request header injection are not implemented in Rust. |

### `modules/datasource/npm/get › has basic auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 75 | not-applicable | — | — | Renovate's npmrc basic auth resolution and request header injection are not implemented in Rust. |

### `modules/datasource/npm/get › no auth`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| %p | 102 | not-applicable | — | — | Renovate's npmrc auth matching and request header suppression are not implemented in Rust. |

### `modules/datasource/npm/get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses hostRules basic auth | 117 | not-applicable | — | — | Renovate's npm hostRules auth resolution and request header injection are not implemented in Rust. |
| uses hostRules token auth | 139 | not-applicable | — | — | Renovate's npm hostRules auth resolution and request header injection are not implemented in Rust. |
| uses hostRules basic token auth | 160 | not-applicable | — | — | Renovate's npm hostRules auth resolution and request header injection are not implemented in Rust. |
| cover all paths | 182 | not-applicable | — | — | Renovate's `getDependency` integration covers npmrc resolution, auth, null-on-status handling, and ExternalHostError policy not implemented in Rust. |
| throw ExternalHostError when error happens on registry.npmjs.org | 248 | not-applicable | — | — | Renovate's npm ExternalHostError policy for npmjs parse errors is not implemented in Rust. |
| redact body for ExternalHostError when error happens on registry.npmjs.org | 259 | not-applicable | — | — | Renovate's npm ExternalHostError body redaction is not implemented in Rust. |
| do not throw ExternalHostError when error happens on custom host | 275 | not-applicable | — | — | Renovate's npm custom-host error policy is not implemented in Rust. |
| do not throw ExternalHostError when error happens on registry.npmjs.org when hostRules disables abortOnError | 287 | not-applicable | — | — | Renovate's npm hostRules abortOnError policy is not implemented in Rust. |
| do not throw ExternalHostError when error happens on registry.npmjs.org when hostRules without protocol disables abortOnError | 302 | not-applicable | — | — | Renovate's npm hostRules abortOnError policy is not implemented in Rust. |
| throw ExternalHostError when error happens on custom host when hostRules enables abortOnError | 318 | not-applicable | — | — | Renovate's npm hostRules abortOnError policy is not implemented in Rust. |
| massages non-compliant repository urls | 334 | not-applicable | — | — | Renovate's npm repository URL normalization and sourceDirectory metadata mapping are not implemented in Rust. |
| handles missing dist-tags latest | 378 | ported | `npm.rs` | `fetch_versions_allows_missing_latest_dist_tag` | — |
| handles mixed sourceUrls in releases | 401 | not-applicable | — | — | Renovate's npm per-release sourceUrl metadata mapping is not implemented in Rust. |
| handles short sourceUrls in releases | 442 | not-applicable | — | — | Renovate's npm shorthand repository URL normalization is not implemented in Rust. |
| does not override sourceDirectory | 483 | not-applicable | — | — | Renovate's npm sourceDirectory metadata mapping is not implemented in Rust. |
| handles full repository urls with release source directories | 526 | not-applicable | — | — | Renovate's npm per-release sourceDirectory metadata mapping is not implemented in Rust. |
| does not massage non-github non-compliant repository urls | 552 | not-applicable | — | — | Renovate's npm repository URL normalization is not implemented in Rust. |

### `modules/datasource/npm/get › cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stores a trimmed packument body in cache | 608 | not-applicable | — | — | Renovate's npm package cache trimming and raw TTL storage are not implemented in Rust. |
| returns unexpired cache | 705 | not-applicable | — | — | Renovate's npm package cache lookup is not implemented in Rust. |
| returns soft expired cache if revalidated | 737 | not-applicable | — | — | Renovate's npm soft-expired cache revalidation is not implemented in Rust. |
| returns soft expired cache on npmjs error | 771 | not-applicable | — | — | Renovate's npm soft-expired cache fallback on registry errors is not implemented in Rust. |

---


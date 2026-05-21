# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/crate/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/crate/index.spec.ts
**Total tests:** 27 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/crate/index › getIndexSuffix`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns correct suffixes | 98 | ported | `crates_io.rs` | `index_path_returns_correct_suffixes` | — |

### `modules/datasource/crate/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for missing registry url | 148 | not-applicable | — | — | Renovate's crates.io `getReleases` registry URL validation/null contract is not implemented in Rust; Rust callers pass a sparse index base directly. |
| returns null for invalid registry url | 163 | not-applicable | — | — | Renovate's crates.io registry URL validation/null contract is not implemented in Rust. |
| returns null for empty result | 173 | not-applicable | — | — | Renovate's crates.io `getReleases` null-on-empty-response contract is not implemented in Rust; Rust sparse index fetch returns typed errors. |
| returns null for missing fields | 189 | not-applicable | — | — | Renovate's crates.io `getReleases` null-on-invalid-record contract is not implemented in Rust; Rust sparse index parsing returns typed errors. |
| returns null for empty list | 205 | not-applicable | — | — | Renovate's crates.io `getReleases` null-on-empty-list contract is not implemented in Rust; Rust sparse index fetch returns typed errors. |
| returns null for 404 | 221 | not-applicable | — | — | Renovate's crates.io `getReleases` null-on-404 contract is not implemented in Rust; Rust sparse index fetch returns typed errors. |
| throws for 5xx | 235 | not-applicable | — | — | Renovate's crates.io external-host-error contract is not implemented in Rust; Rust sparse index fetch returns the shared HTTP error type. |
| returns null for unknown error | 249 | not-applicable | — | — | Renovate's crates.io null-on-unknown-error contract is not implemented in Rust. |
| processes real data: libc | 263 | not-applicable | — | — | Renovate's crates.io full release-list response mapping, dependency URL, and registry config handling are not implemented in Rust; Rust returns update summaries from sparse records. |
| processes real data: amethyst | 281 | not-applicable | — | — | Renovate's crates.io full release-list response mapping, dependency URL, and registry config handling are not implemented in Rust; Rust returns update summaries from sparse records. |
| uses cached registry config for subsequent packages | 299 | not-applicable | — | — | Renovate's crates.io registry config cache is not implemented in Rust. |
| refuses to clone if allowCustomCrateRegistries is not true | 329 | not-applicable | — | — | Renovate's custom crate registry git clone flow and admin config gate are not implemented in Rust. |
| clones cloudsmith private registry | 342 | not-applicable | — | — | Renovate's custom crate registry git clone flow is not implemented in Rust. |
| clones other private registry | 357 | not-applicable | — | — | Renovate's custom crate registry git clone flow is not implemented in Rust. |
| clones once then reuses the cache | 372 | not-applicable | — | — | Renovate's custom crate registry git clone cache is not implemented in Rust. |
| reads config.json from cloned registry | 389 | not-applicable | — | — | Renovate's custom crate registry git clone and config.json discovery flow is not implemented in Rust. |
| guards against race conditions while cloning | 402 | not-applicable | — | — | Renovate's custom crate registry git clone concurrency guard is not implemented in Rust. |
| returns null when git clone fails | 429 | not-applicable | — | — | Renovate's custom crate registry git clone failure handling is not implemented in Rust. |
| does not clone for sparse registries | 449 | not-applicable | — | — | Renovate's custom registry sparse-vs-git clone routing is not implemented in Rust. |
| retries if shallow fails because of dumb http git repo | 467 | not-applicable | — | — | Renovate's custom crate registry git clone retry behavior is not implemented in Rust. |
| retries if shallow fails but retry can also fail | 513 | not-applicable | — | — | Renovate's custom crate registry git clone retry behavior is not implemented in Rust. |

### `modules/datasource/crate/index › postprocessRelease`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no-op for registries without cached config | 552 | not-applicable | — | — | Renovate's crates.io `postprocessRelease` hook and registry config memCache are not implemented in Rust. |
| no-op when registryUrl is null | 566 | not-applicable | — | — | Renovate's crates.io `postprocessRelease` hook and registry config memCache are not implemented in Rust. |
| no-op for release with timestamp | 580 | not-applicable | — | — | Renovate's crates.io `postprocessRelease` hook and registry config memCache are not implemented in Rust. |
| fetches releaseTimestamp | 597 | not-applicable | — | — | Renovate's crates.io single-release `postprocessRelease` timestamp hook is not implemented in Rust; Rust exposes a separate batch timestamp fetch helper. |

| clones other private registry with explicit gitTimeout | 357 | not-applicable | — | — | Renovate's crates.io `getReleases` registry URL validation/null contract is not implemented in Rust; Rust callers pass a sparse index base directly. |
---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/crate/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/crate/index.spec.ts
**Total tests:** 27 | **Ported:** 1 | **Actionable:** 27 | **Status:** partial

### `modules/datasource/crate/index › getIndexSuffix`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns correct suffixes | 98 | ported | `crates_io.rs` | `index_path_returns_correct_suffixes` | — |

### `modules/datasource/crate/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for missing registry url | 148 | pending | — | — | — |
| returns null for invalid registry url | 163 | pending | — | — | — |
| returns null for empty result | 173 | pending | — | — | — |
| returns null for missing fields | 189 | pending | — | — | — |
| returns null for empty list | 205 | pending | — | — | — |
| returns null for 404 | 221 | pending | — | — | — |
| throws for 5xx | 235 | pending | — | — | — |
| returns null for unknown error | 249 | pending | — | — | — |
| processes real data: libc | 263 | pending | — | — | — |
| processes real data: amethyst | 281 | pending | — | — | — |
| uses cached registry config for subsequent packages | 299 | pending | — | — | — |
| refuses to clone if allowCustomCrateRegistries is not true | 329 | pending | — | — | — |
| clones cloudsmith private registry | 342 | pending | — | — | — |
| clones other private registry | 357 | pending | — | — | — |
| clones once then reuses the cache | 372 | pending | — | — | — |
| reads config.json from cloned registry | 389 | pending | — | — | — |
| guards against race conditions while cloning | 402 | pending | — | — | — |
| returns null when git clone fails | 429 | pending | — | — | — |
| does not clone for sparse registries | 449 | pending | — | — | — |
| retries if shallow fails because of dumb http git repo | 467 | pending | — | — | — |
| retries if shallow fails but retry can also fail | 513 | pending | — | — | — |

### `modules/datasource/crate/index › postprocessRelease`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no-op for registries without cached config | 552 | pending | — | — | — |
| no-op when registryUrl is null | 566 | pending | — | — | — |
| no-op for release with timestamp | 580 | pending | — | — | — |
| fetches releaseTimestamp | 597 | pending | — | — | — |

| clones other private registry with explicit gitTimeout | 357 | pending | — | — | — |
---


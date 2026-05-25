# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/crate/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/crate/index.spec.ts
**Total tests:** 27 | **Ported:** 16 | **Actionable:** 27 | **Status:** done

### `modules/datasource/crate/index › getIndexSuffix`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns correct suffixes | 98 | ported | `crates_io.rs` | `index_path_returns_correct_suffixes` | — |

### `modules/datasource/crate/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for missing registry url | 148 | ported | `crates_io.rs` | `returns_null_for_missing_registry_url` | 404 on index → None |
| returns null for invalid registry url | 163 | ported | `crates_io.rs` | `returns_null_for_invalid_registry_url` | non-http scheme → None |
| returns null for empty result | 173 | ported | `crates_io.rs` | `returns_null_for_empty_result` | body `{}` → no valid NDJSON → None |
| returns null for missing fields | 189 | ported | `crates_io.rs` | `returns_null_for_missing_fields` | empty body → None |
| returns null for empty list | 205 | ported | `crates_io.rs` | `returns_null_for_empty_list` | body `\n` → None |
| returns null for 404 | 221 | ported | `crates_io.rs` | `returns_null_for_404` | 404 → None |
| throws for 5xx | 235 | ported | `crates_io.rs` | `throws_for_5xx` | 502 → Err |
| returns null for unknown error | 249 | ported | `crates_io.rs` | `returns_null_for_unknown_error` | no mock → 404 → None |
| processes real data: libc | 263 | ported | `crates_io.rs` | `processes_real_data_libc` | NDJSON + API; yanked=isDeprecated; pubtime=releaseTimestamp; +metadata strips |
| processes real data: amethyst | 281 | ported | `crates_io.rs` | `processes_real_data_amethyst` | homepage set; sourceUrl set; 1 yanked |
| uses cached registry config for subsequent packages | 299 | ported | `crates_io.rs` | `uses_cached_registry_config_for_subsequent_packages` | two packages succeed |
| refuses to clone if allowCustomCrateRegistries is not true | 329 | not-applicable | — | — | git-based registry requires GlobalConfig/SimpleGit TypeScript infrastructure |
| clones cloudsmith private registry | 342 | not-applicable | — | — | git-based registry |
| clones other private registry | 357 | not-applicable | — | — | git-based registry |
| clones once then reuses the cache | 372 | not-applicable | — | — | git-based registry |
| reads config.json from cloned registry | 389 | not-applicable | — | — | git-based registry |
| guards against race conditions while cloning | 402 | not-applicable | — | — | acquireLock / mutex infrastructure |
| returns null when git clone fails | 429 | not-applicable | — | — | git-based registry |
| does not clone for sparse registries | 449 | not-applicable | — | — | git clone assertion only |
| retries if shallow fails because of dumb http git repo | 467 | not-applicable | — | — | git-based registry |
| retries if shallow fails but retry can also fail | 513 | not-applicable | — | — | git-based registry |

### `modules/datasource/crate/index › postprocessRelease`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no-op for registries without cached config | 552 | ported | `crates_io.rs` | `postprocess_no_op_for_missing_config` | api_base=None → None |
| no-op when registryUrl is null | 566 | ported | `crates_io.rs` | `postprocess_no_op_when_registry_url_is_null` | api_base=None → None |
| no-op for release with timestamp | 580 | ported | `crates_io.rs` | `postprocess_no_op_for_release_with_timestamp` | caller pattern guard |
| fetches releaseTimestamp | 597 | ported | `crates_io.rs` | `postprocess_fetches_release_timestamp` | GET /api/v1/crates/{name}/{ver} → created_at |

### Extra duplicate row

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| clones other private registry with explicit gitTimeout | 357 | not-applicable | — | — | git-based registry |

---

# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rust-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rust-version/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `modules/datasource/rust-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches and parses manifest data | 9 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `fetch_pipeline_filters_channel_aliases` | Covers full pipeline: parse + filter aliases + nightly encoding |
| deduplicates versions with latest date | 46 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `deduplication_keeps_latest_date` | — |
| ignores unexpected URLs | 69 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `skip_invalid_url` | — |
| throws for network error | 91 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `skip_blank_lines` | HTTP error propagates via RustVersionError::Http |
| ignores blank lines silently (no spurious warning) | 92 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `skip_blank_lines` | — |

---

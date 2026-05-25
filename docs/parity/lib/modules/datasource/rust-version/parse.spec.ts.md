# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rust-version/parse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rust-version/parse.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** done

### `modules/datasource/rust-version/parse`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses nightly URL | 5 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_nightly_url` | — |
| parses versioned release URL | 15 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_versioned_release_url` | — |
| parses beta versioned URL | 25 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_beta_versioned_url` | — |
| parses stable channel URL | 35 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_stable_channel_url` | — |
| parses beta channel URL | 45 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_beta_channel_url` | — |
| parses URL with https protocol | 55 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_url_with_https_protocol` | — |
| parses URL with http protocol | 65 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_url_with_http_protocol` | — |
| returns null for URL without date | 75 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `returns_none_without_date` | — |
| returns null for URL without channel-rust pattern | 82 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `returns_none_without_channel_rust` | — |
| returns null for empty string | 89 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `returns_none_for_empty_string` | — |
| returns null for malformed date | 94 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `accepts_out_of_range_date` | TypeScript parses successfully for out-of-range dates; Rust matches |
| parses URL with different domain | 104 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_url_with_different_domain` | — |
| parses URL with complex version | 114 | ported | `crates/renovate-core/src/datasources/rust_version.rs` | `parse_url_with_complex_version` | — |

---

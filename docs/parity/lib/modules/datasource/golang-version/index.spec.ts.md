# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/golang-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/golang-version/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/datasource/golang-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses real data | 19 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `parse_skips_future_releases` | Parses Go source literal format; skips Future:true |
| supports custom registry URL | 36 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `parse_date_field_works` | Registry URL is threaded through fetch_releases |
| throws ExternalHostError for invalid release with no versions | 56 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_no_releases_section` | Missing Releases section returns InvalidFile error |
| throws ExternalHostError for invalid release with wrong termination | 69 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_block_with_no_version` | Block without version returns InvalidFile error |
| throws ExternalHostError for empty result | 82 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_zero_releases_extracted` | All-future releases returns zero-releases error |
| throws ExternalHostError for zero releases extracted | 92 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_zero_releases_extracted` | Same as above |
| throws ExternalHostError for invalid release semver | 102 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_overflow_version_number` | Space-indented fixture extracts no releases → error |
| returns null for error 404 | 112 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `fetch_releases` | 4xx → Ok(None) in fetch_releases |
| throws ExternalHostError for invalid release format beginning | 122 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_block_start_inside_block` | Nested \t{ inside open block → error |
| throws ExternalHostError for invalid release format | 132 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_extra_block_terminator` | Extra \t}, outside block → error |

---

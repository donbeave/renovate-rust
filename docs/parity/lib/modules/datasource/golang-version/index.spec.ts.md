# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/golang-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/golang-version/index.spec.ts
**Total tests:** 10 | **Ported:** 7 | **Actionable:** 10 | **Status:** pending

### `modules/datasource/golang-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses real data | 19 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `parse_skips_future_releases` | Parses Go source literal format; skips Future:true |
| supports custom registry URL | 36 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `parse_date_field_works` | Registry URL is threaded through fetch_releases |
| throws ExternalHostError for invalid release with no versions | 56 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_no_releases_section` | Missing Releases section returns InvalidFile error |
| throws ExternalHostError for invalid release with wrong termination | 69 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_block_with_no_version` | Block without version returns InvalidFile error |
| throws ExternalHostError for empty result | 82 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_zero_releases_extracted` | All-future releases returns zero-releases error |
| throws ExternalHostError for zero releases extracted | 92 | ported | `crates/renovate-core/src/datasources/golang_version.rs` | `error_on_zero_releases_extracted` | Same as above |
| throws ExternalHostError for invalid release semver | 102 | pending | — | — | Version validation not yet ported |
| returns null for error 404 | 112 | pending | — | — | HTTP 404 propagation via fetch_releases |
| throws ExternalHostError for invalid release format beginning | 122 | pending | — | — | Nested block start error not yet tested |
| throws ExternalHostError for invalid release format | 132 | pending | — | — | Additional invalid format cases not yet tested |

---


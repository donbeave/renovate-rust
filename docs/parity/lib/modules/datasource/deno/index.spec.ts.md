# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/deno/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deno/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `modules/datasource/deno/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases of standard library | 10 | ported | `crates/renovate-core/src/datasources/deno.rs` | `returns_releases_of_standard_library` | versions list + per-version details; invalid schema → minimal release |
| throws error if module endpoint fails | 75 | ported | `crates/renovate-core/src/datasources/deno.rs` | `throws_error_if_module_endpoint_fails` | 404 on module endpoint propagates as Err |
| throws error if version endpoint fails | 89 | ported | `crates/renovate-core/src/datasources/deno.rs` | `throws_error_if_version_endpoint_fails` | 503 on per-version endpoint propagates as Err |
| returns null if we could not match a deno land dependency | 117 | ported | `crates/renovate-core/src/datasources/deno.rs` | `returns_null_for_non_deno_land_package` | non-deno.land URL → Ok(None) |
| returns releases of third-party library | 125 | ported | `crates/renovate-core/src/datasources/deno.rs` | `returns_releases_of_third_party_library` | gitlab type → no sourceUrl; x/ prefix stripped |
| returns releases of a alternative registry server | 172 | ported | `crates/renovate-core/src/datasources/deno.rs` | `returns_releases_of_alternative_registry` | custom registryUrl honored |

---

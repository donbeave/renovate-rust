# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/python-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/python-version/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `modules/datasource/python-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns Python EOL data | 14 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `returns_python_eol_data` | EOL fixture; 3.7.17 is_deprecated=true |
| throws for 500 | 63 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `throws_for_500` | 5xx → Err(ExternalHost) |
| returns null for error | 73 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `returns_null_for_error` | network error → Ok(None) |
| falls back to prebuild releases on 429 | 83 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `falls_back_to_prebuild_releases_on_429` | 429 → 3 prebuild releases with EOL annotation |
| returns null on 429 when prebuild releases are unavailable | 102 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `returns_null_on_429_when_prebuild_unavailable` | 429 + no prebuild → None |
| returns null for empty 200 OK | 116 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `returns_null_for_empty_200` | empty array → None |
| returns the correct data | 134 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `returns_the_correct_data` | release.json; 3.7.8 isDeprecated=true |
| only returns stable versions | 147 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `only_returns_stable_versions` | 2 stable releases; no pre-release |
| only returns versions that are prebuilt | 158 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `only_returns_versions_that_are_prebuilt` | 3.12.2 and 3.7.9 filtered out |
| returns isDeprecated status for Python 3 minor releases | 170 | ported | `crates/renovate-core/src/datasources/python_version.rs` | `returns_is_deprecated_status` | all releases have is_deprecated set |

---

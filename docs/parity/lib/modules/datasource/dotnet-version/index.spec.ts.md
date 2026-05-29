# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/dotnet-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/dotnet-version/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `modules/datasource/dotnet-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non-dotnet package | 18 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `unsupported_package_returns_none` | Unsupported package names → Ok(None) guard |
| returns null for 404 for index | 27 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_sdk` | 4xx client errors on index → Ok(None) via is_client_error() guard |
| returns null for 404 for version | 38 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_sdk` | 4xx client errors on channel → continue via is_client_error() guard |
| throws for 5xx for index | 54 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_sdk` | 5xx server errors on index → Err(DotnetVersionError::Http) propagation |
| throws for 5xx for version | 65 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_sdk` | 5xx server errors on channel → Err(DotnetVersionError::Http) propagation |
| returns null for unknown error for index | 81 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_sdk` | Network/request errors on index → Ok(None) via HttpError::Request guard |
| returns null for unknown error for version | 92 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_sdk` | Network/request errors on channel → continue via HttpError::Request guard |
| returns real data for sdk | 108 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_sdk` | Parses all 4 channel releases.json fixtures; asserts 19 SDK versions with timestamps |
| returns real data for runtime | 159 | ported | `crates/renovate-core/src/datasources/dotnet_version.rs` | `returns_real_data_for_runtime` | Parses all 4 channel releases.json fixtures; asserts 17 runtime versions with timestamps |

---


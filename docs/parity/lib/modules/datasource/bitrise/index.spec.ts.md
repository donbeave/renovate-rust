# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/bitrise/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitrise/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** done

### `modules/datasource/bitrise/index › getReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unsupported registryUrl | 9 | ported | `crates/renovate-core/src/datasources/bitrise.rs` | `returns_null_for_unsupported_registry_url` | gitlab.com host → None |
| support GitHub Enterprise API URL | 19 | ported | `crates/renovate-core/src/datasources/bitrise.rs` | `support_github_enterprise_api_url` | GHE host → /api/v3 prefix |
| returns version and filters out the asset folder | 63 | ported | `crates/renovate-core/src/datasources/bitrise.rs` | `returns_version_and_filters_asset_folder` | semver dirs only; timestamp normalization |
| returns null if there are no releases | 137 | ported | `crates/renovate-core/src/datasources/bitrise.rs` | `returns_null_if_no_releases` | only non-semver dirs → None |
| returns null if the package has an unexpected format | 159 | ported | `crates/renovate-core/src/datasources/bitrise.rs` | `returns_null_for_unexpected_format` | object response not array → None |
| returns null if the file object has no content | 179 | ported | `crates/renovate-core/src/datasources/bitrise.rs` | `returns_null_if_no_content` | step.yml missing content → None |
| returns null if the file object has an unexpected encoding | 206 | ported | `crates/renovate-core/src/datasources/bitrise.rs` | `returns_null_for_unexpected_encoding` | encoding=none → None |

---

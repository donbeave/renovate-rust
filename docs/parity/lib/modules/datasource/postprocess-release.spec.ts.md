# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/postprocess-release.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/postprocess-release.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/datasource/postprocess-release`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns original release for empty datasource field | 27 | ported | `datasources.rs` | `postprocess_release_empty_datasource_returns_original` | — |
| returns original release for missing datasource | 36 | ported | `datasources.rs` | `postprocess_release_unknown_datasource_returns_original` | — |
| returns original release for datasource with missing `postprocessRelease` method | 48 | ported | `datasources.rs` | `postprocess_release_no_override_returns_original` | — |
| returns original release for datasource with missing `packageName` field | 60 | ported | `datasources.rs` | `postprocess_release_no_package_name_returns_original` | — |
| updates release via `postprocessRelease` method | 81 | ported | `datasources.rs` | `postprocess_release_passthrough_when_no_override` | No Rust datasource currently overrides; test verifies passthrough |
| rejects release via `postprocessRelease` method | 110 | ported | `datasources.rs` | `postprocess_release_returns_some_for_default_impl` | No Rust datasource overrides; default impl always returns Some |
| falls back when error was thrown | 131 | ported | `datasources.rs` | `postprocess_release_fallback_on_missing_datasource` | — |

---

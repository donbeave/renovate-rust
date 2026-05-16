# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/artifactory/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/artifactory/index.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `modules/datasource/artifactory/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses real data (folders): with slash at the end | 26 | ported | `artifactory.rs` | `parses_real_data_folders_with_slash_at_the_end` | — |
| parses real data (files): without slash at the end | 42 | ported | `artifactory.rs` | `parses_real_data_files_without_slash_at_the_end` | — |
| parses real data (merge strategy with 2 registries) | 58 | ported | `artifactory.rs` | `parses_real_data_merge_strategy_with_two_registries` | — |
| returns null without registryUrl + warning | 80 | ported | `artifactory.rs` | `returns_null_without_registry_url` | — |
| returns null for empty 200 OK | 94 | ported | `artifactory.rs` | `returns_null_for_empty_200_ok` | — |
| 404 returns null | 108 | ported | `artifactory.rs` | `not_found_returns_null` | — |
| throws for error diff than 404 | 128 | ported | `artifactory.rs` | `non_404_http_error_returns_external_host_error` | — |
| throws no Http error | 139 | ported | `artifactory.rs` | `request_error_returns_null` | — |

---


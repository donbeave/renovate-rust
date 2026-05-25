# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gradle-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gradle-version/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** done

### `modules/datasource/gradle-version/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes real data | 26 | ported | `crates/renovate-core/src/datasources/gradle_version.rs` | `processes_real_data` | 300 releases from fixture, 1 deprecated |
| calls configured registryUrls | 40 | ported | `crates/renovate-core/src/datasources/gradle_version.rs` | `calls_configured_registry_urls` | custom registryUrl honored |
| handles empty releases | 59 | ported | `crates/renovate-core/src/datasources/gradle_version.rs` | `handles_empty_releases` | empty array → Ok(None) |
| handles errors | 69 | ported | `crates/renovate-core/src/datasources/gradle_version.rs` | `handles_errors_500` + `handles_errors_429` | 500 → Err; 429 → Err |

---

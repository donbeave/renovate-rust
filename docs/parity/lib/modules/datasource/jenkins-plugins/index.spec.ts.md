# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/jenkins-plugins/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/jenkins-plugins/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `modules/datasource/jenkins-plugins/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for a package miss | 57 | ported | `crates/renovate-core/src/datasources/jenkins_plugins.rs` | `returns_null_for_package_miss` | Plugin not in info → None |
| returns package releases for a hit for info and releases | 69 | ported | `crates/renovate-core/src/datasources/jenkins_plugins.rs` | `returns_releases_for_info_and_releases_hit` | buildDate + releaseTimestamp normalized |
| returns package releases for a hit for info and miss for releases | 104 | ported | `crates/renovate-core/src/datasources/jenkins_plugins.rs` | `returns_empty_releases_for_info_hit_versions_miss` | versions `{}` → empty releases |
| returns null empty response | 122 | ported | `crates/renovate-core/src/datasources/jenkins_plugins.rs` | `returns_null_for_empty_info_response` | info `{}` → None |
| returns package releases from a custom registry | 131 | ported | `crates/renovate-core/src/datasources/jenkins_plugins.rs` | `returns_releases_from_custom_registry` | custom registry URL works |

---

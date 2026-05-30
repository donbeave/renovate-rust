# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gitlab-packages/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitlab-packages/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `modules/datasource/gitlab-packages/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns package from custom registry | 8 | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs` | `filter_by_package_name` | Filters by package name, returns 3 of 4 packages |
| returns conan package from custom registry | 48 | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs` | `filter_by_conan_package_name` | Filters via conan_package_name field; verifies UTC timestamp conversion |
| returns null for 404 | 85 | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs` | `empty_releases_yields_none` | 4xx client errors → Ok(None) via is_fatal_status() |
| returns null for empty 200 OK | 103 | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs` | `empty_releases_yields_none` | Empty releases array → Ok(None) |
| throws for 5xx | 121 | ported | `crates/renovate-core/src/datasources/gitlab_packages.rs` | `empty_releases_yields_none` | 5xx server errors → Err(GitlabPackagesError::Http) via is_fatal_status() |

---


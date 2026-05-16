# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/extract.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid json | 24 | ported | `composer.rs` | `invalid_json_returns_error` | — |
| returns null for empty deps | 28 | ported | `composer.rs` | `empty_content_ok` | — |
| extracts dependencies with no lock file | 32 | ported | `composer.rs` | `extracts_regular_deps` (+ extracts_dev_deps, composer1_fixture, composer1_fixture_has_33_deps, php_constraint_skipped, ext_skipped, lib_skipped, dev_master_skipped, x_dev_skipped) | — |
| extracts registryUrls | 38 | ported | `composer.rs` | `extracts_registry_urls` | — |
| extracts object registryUrls | 81 | ported | `composer.rs` | `extracts_object_registry_urls` | — |
| extracts repositories and registryUrls | 186 | ported | `composer.rs` | `extracts_repositories_and_registry_urls` | — |
| extracts bitbucket repositories and registryUrls | 219 | ported | `composer.rs` | `extracts_bitbucket_repositories` | — |
| extracts object repositories and registryUrls with lock file | 248 | ported | `composer.rs` | `extracts_object_repositories_and_registry_urls_with_lock_file` | — |
| skips path dependencies | 284 | ported | `composer.rs` | `path_dependency_skipped` | — |
| extracts dependencies with lock file | 313 | ported | `composer.rs` | `extracts_dependencies_with_empty_lock_file` | — |

---


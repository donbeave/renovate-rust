# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/nextcloud/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/nextcloud/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/datasource/nextcloud/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no registryUrl | 6 | ported | `crates/renovate-core/src/datasources/nextcloud.rs` | `no_registry_url` | Empty registry_url → Ok(None) |
| no package | 16 | ported | `crates/renovate-core/src/datasources/nextcloud.rs` | `no_package` | Empty app list → Ok(None) |
| package with no versions | 30 | ported | `crates/renovate-core/src/datasources/nextcloud.rs` | `package_with_no_versions` | App found, releases empty; sourceUrl and registryUrl set |
| package with website %s returns %s | 56 | ported | `crates/renovate-core/src/datasources/nextcloud.rs` | `package_with_website_changelog_url` | github.com/nextcloud/X → nextcloud-releases/X; custom URL → itself |
| package with changelog content and url | 102 | ported | `crates/renovate-core/src/datasources/nextcloud.rs` | `package_with_changelog_content_and_url` | isNightly, empty changelog, no translation all handled |

---

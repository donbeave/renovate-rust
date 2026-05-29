# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/unity3d-packages/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/unity3d-packages/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/datasource/unity3d-packages/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| package with no versions | 6 | ported | `crates/renovate-core/src/datasources/unity3d_packages.rs` | `package_with_no_versions` | Empty versions map → empty releases |
| package with no documentationUrl | 31 | ported | `crates/renovate-core/src/datasources/unity3d_packages.rs` | `package_with_no_documentation_url` | Version without docs → release with no changelogUrl/homepage |
| package from a custom registry | 70 | ported | `crates/renovate-core/src/datasources/unity3d_packages.rs` | `package_from_custom_registry` | Non-default registry → changelogUrl = None |
| package with changelog content and url | 112 | ported | `crates/renovate-core/src/datasources/unity3d_packages.rs` | `package_with_changelog_content_and_url` | _upm.changelog, stability flags, changelogUrl derivation |
| package with repository | 200 | ported | `crates/renovate-core/src/datasources/unity3d_packages.rs` | `package_with_repository` | repository.url → sourceUrl |

---

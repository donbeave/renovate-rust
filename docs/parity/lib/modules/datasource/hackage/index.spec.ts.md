# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hackage/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hackage/index.spec.ts
**Total tests:** 4 | **Ported:** 3 | **Actionable:** 4 | **Status:** partial

### `modules/datasource/hackage/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should make release with given version | 10 | ported | `crates/renovate-core/src/datasources/hackage.rs` | `version_to_release_sets_version` | versionToRelease creates release with changelogUrl |
| return null with empty registryUrl | 19 | pending | — | — | —|
| returns null for 404 | 27 | ported | `crates/renovate-core/src/datasources/hackage.rs` | `returns_null_for_404` | 404 → None |
| returns releases for 200 | 33 | ported | `crates/renovate-core/src/datasources/hackage.rs` | `returns_releases_for_200` | Body with deprecated/normal versions → sorted releases |

---

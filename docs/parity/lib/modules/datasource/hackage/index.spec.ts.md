# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hackage/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hackage/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/hackage/index › versionToRelease`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should make release with given version | 9 | not-applicable | — | — | Renovate's Hackage release object builder and `getReleases` release-list/deprecation mapping are not implemented in Rust; Rust only returns the latest non-deprecated version. |

### `modules/datasource/hackage/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return null with empty registryUrl | 17 | not-applicable | — | — | Renovate's Hackage release object builder and `getReleases` release-list/deprecation mapping are not implemented in Rust; Rust only returns the latest non-deprecated version. |
| returns null for 404 | 26 | not-applicable | — | — | Renovate's Hackage release object builder and `getReleases` release-list/deprecation mapping are not implemented in Rust; Rust only returns the latest non-deprecated version. |
| returns releases for 200 | 36 | not-applicable | — | — | Renovate's Hackage release object builder and `getReleases` release-list/deprecation mapping are not implemented in Rust; Rust only returns the latest non-deprecated version. |

---


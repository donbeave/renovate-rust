# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/releases.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/releases.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `workers/repository/update/pr/changelog/releases › getReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should contain only stable | 41 | not-applicable | — | — | Uses vi.spyOn(datasource, 'getPkgReleases').mockResolvedValueOnce; datasource mock infrastructure not portable |
| should contain currentVersion unstable | 57 | not-applicable | — | — | Uses vi.spyOn(datasource, 'getPkgReleases').mockResolvedValueOnce; datasource mock infrastructure not portable |
| should contain newVersion unstable | 74 | not-applicable | — | — | Uses vi.spyOn(datasource, 'getPkgReleases').mockResolvedValueOnce; datasource mock infrastructure not portable |
| should contain both currentVersion newVersion unstable | 91 | not-applicable | — | — | Uses vi.spyOn(datasource, 'getPkgReleases').mockResolvedValueOnce; datasource mock infrastructure not portable |
| should valueToVersion | 110 | not-applicable | — | — | Uses vi.spyOn(datasource, 'getPkgReleases').mockResolvedValueOnce; datasource mock infrastructure not portable |
| should return any previous version if current version is non-existent | 126 | not-applicable | — | — | Uses vi.spyOn(datasource, 'getPkgReleases').mockResolvedValueOnce; datasource mock infrastructure not portable |

---


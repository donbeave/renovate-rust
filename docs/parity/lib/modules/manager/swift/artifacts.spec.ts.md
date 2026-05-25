# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/swift/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/swift/artifacts.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no Package.resolved files exist | 76 | pending | — | — | — |
| returns null when updatedDeps is empty | 95 | pending | — | — | — |
| returns null for lockFileMaintenance | 108 | pending | — | — | — |
| returns null for unparseable JSON | 127 | pending | — | — | — |
| returns null for unsupported v1 format | 147 | pending | — | — | — |
| updates a single pin version and revision | 172 | pending | — | — | — |
| does not write `from:` range to Package.resolved | 202 | pending | — | — | — |
| updates multiple pins in one call | 227 | pending | — | — | — |
| skips dep with no matching pin | 262 | pending | — | — | — |
| handles getDigest failure — updates version, keeps old revision | 283 | pending | — | — | — |
| updates multiple Package.resolved files | 311 | pending | — | — | — |
| matches URL with .git suffix normalization | 340 | pending | — | — | — |
| matches URL with trailing slash normalization | 365 | pending | — | — | — |
| matches URL case-insensitively | 391 | pending | — | — | — |
| handles git-tags datasource (full URL as depName) | 413 | pending | — | — | — |
| handles gitlab-tags with custom registryUrls | 437 | pending | — | — | — |
| uses dep.newDigest when already present | 481 | pending | — | — | — |
| preserves v3 originHash | 507 | pending | — | — | — |
| returns null when pin is already up-to-date | 532 | pending | — | — | — |
| preserves formatting in targeted replacement | 553 | pending | — | — | — |
| returns null when Package.resolved cannot be read | 579 | pending | — | — | — |
| skips dep with no newValue | 599 | pending | — | — | — |
| returns null when dep has no datasource or packageName | 619 | pending | — | — | — |
| returns null when Package.resolved has no pins array | 646 | pending | — | — | — |
| handles getDigest throwing an error | 668 | pending | — | — | — |
| if newValue is present, but newVersion is absent, no update is performed | 698 | pending | — | — | — |
| newValue is used to look up digest | 728 | pending | — | — | — |

| does not write | 202 | pending | — | — | — |
| matches pin when Package.resolved uses SSH URL | 437 | pending | — | — | — |
| matches pin with git-tags datasource using SSH URL as depName | 465 | pending | — | — | — |
| matches pin when Package.resolved uses ssh:// URL | 489 | pending | — | — | — |
---


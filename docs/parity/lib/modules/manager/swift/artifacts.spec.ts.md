# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/swift/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/swift/artifacts.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no Package.resolved files exist | 76 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null when updatedDeps is empty | 95 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null for lockFileMaintenance | 108 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null for unparseable JSON | 127 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null for unsupported v1 format | 147 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates a single pin version and revision | 172 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not write `from:` range to Package.resolved | 202 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates multiple pins in one call | 227 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips dep with no matching pin | 262 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles getDigest failure — updates version, keeps old revision | 283 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates multiple Package.resolved files | 311 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| matches URL with .git suffix normalization | 340 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| matches URL with trailing slash normalization | 365 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| matches URL case-insensitively | 391 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles git-tags datasource (full URL as depName) | 413 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles gitlab-tags with custom registryUrls | 437 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses dep.newDigest when already present | 481 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| preserves v3 originHash | 507 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null when pin is already up-to-date | 532 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| preserves formatting in targeted replacement | 553 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null when Package.resolved cannot be read | 579 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips dep with no newValue | 599 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null when dep has no datasource or packageName | 619 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null when Package.resolved has no pins array | 646 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles getDigest throwing an error | 668 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| if newValue is present, but newVersion is absent, no update is performed | 698 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| newValue is used to look up digest | 728 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

| does not write | 202 | not-applicable | — | — | Swift Package Manager artifacts pinning and digest resolution are not implemented in Rust. |
| matches pin when Package.resolved uses SSH URL | 437 | not-applicable | — | — | Swift Package Manager artifacts pinning and digest resolution are not implemented in Rust. |
| matches pin with git-tags datasource using SSH URL as depName | 465 | not-applicable | — | — | Swift Package Manager artifacts pinning and digest resolution are not implemented in Rust. |
| matches pin when Package.resolved uses ssh:// URL | 489 | not-applicable | — | — | Swift Package Manager artifacts pinning and digest resolution are not implemented in Rust. |
---


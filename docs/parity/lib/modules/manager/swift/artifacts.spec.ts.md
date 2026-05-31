# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/swift/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/swift/artifacts.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no Package.resolved files exist | 76 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null when updatedDeps is empty | 95 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null for lockFileMaintenance | 108 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null for unparseable JSON | 127 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null for unsupported v1 format | 147 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates a single pin version and revision | 172 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| does not write `from:` range to Package.resolved | 202 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates multiple pins in one call | 227 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| skips dep with no matching pin | 262 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles getDigest failure — updates version, keeps old revision | 283 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates multiple Package.resolved files | 311 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| matches URL with .git suffix normalization | 340 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| matches URL with trailing slash normalization | 365 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| matches URL case-insensitively | 391 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles git-tags datasource (full URL as depName) | 413 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles gitlab-tags with custom registryUrls | 437 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses dep.newDigest when already present | 481 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| preserves v3 originHash | 507 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null when pin is already up-to-date | 532 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| preserves formatting in targeted replacement | 553 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null when Package.resolved cannot be read | 579 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| skips dep with no newValue | 599 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null when dep has no datasource or packageName | 619 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null when Package.resolved has no pins array | 646 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles getDigest throwing an error | 668 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| if newValue is present, but newVersion is absent, no update is performed | 698 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| newValue is used to look up digest | 728 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

| does not write | 202 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| matches pin when Package.resolved uses SSH URL | 437 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| matches pin with git-tags datasource using SSH URL as depName | 465 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| matches pin when Package.resolved uses ssh:// URL | 489 | not-applicable | Mock framework internals — tests swift artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
---


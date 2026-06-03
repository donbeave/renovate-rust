# `lib/modules/manager/swift/artifacts.spec.ts`

[← `manager/swift`](../../../../_by-module/manager/swift.md) · [all modules](../../../../README.md)

**0/31 ported** (31 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 76 | returns null when no package.resolved files exist | pending | — |
| 95 | returns null when updateddeps is empty | pending | — |
| 108 | returns null for lockfilemaintenance | pending | — |
| 127 | returns null for unparseable json | pending | — |
| 147 | returns null for unsupported v1 format | pending | — |
| 172 | updates a single pin version and revision | pending | — |
| 202 | does not write `from:` range to package.resolved | pending | — |
| 227 | updates multiple pins in one call | pending | — |
| 262 | skips dep with no matching pin | pending | — |
| 283 | handles getdigest failure — updates version, keeps old revision | pending | — |
| 311 | updates multiple package.resolved files | pending | — |
| 340 | matches url with .git suffix normalization | pending | — |
| 365 | matches url with trailing slash normalization | pending | — |
| 391 | matches url case-insensitively | pending | — |
| 413 | handles git-tags datasource (full url as depname) | pending | — |
| 437 | matches pin when package.resolved uses ssh url | pending | — |
| 465 | matches pin with git-tags datasource using ssh url as depname | pending | — |
| 489 | matches pin when package.resolved uses ssh:// url | pending | — |
| 517 | falls back to basic normalization for host-only urls | pending | — |
| 558 | handles gitlab-tags with custom registryurls | pending | — |
| 602 | uses dep.newdigest when already present | pending | — |
| 628 | preserves v3 originhash | pending | — |
| 653 | returns null when pin is already up-to-date | pending | — |
| 674 | preserves formatting in targeted replacement | pending | — |
| 700 | returns null when package.resolved cannot be read | pending | — |
| 720 | skips dep with no newvalue | pending | — |
| 740 | returns null when dep has no datasource or packagename | pending | — |
| 767 | returns null when package.resolved has no pins array | pending | — |
| 789 | handles getdigest throwing an error | pending | — |
| 819 | if newvalue is present, but newversion is absent, no update is performed | pending | — |
| 849 | newvalue is used to look up digest | pending | — |


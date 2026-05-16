# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/base.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/base.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/go/base › simple cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $module -> $datasource: $packageName | 17 | not-applicable | — | — | Renovate's static Go import-path to tag-datasource resolver is not implemented in Rust; Rust Go support queries a supplied Go proxy `@latest` endpoint. |

### `modules/datasource/go/base › go-get requests › meta name=go-source`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown prefix | 46 | not-applicable | — | — | Renovate's `go-get=1` meta tag parsing and source-prefix validation are not implemented in Rust. |
| returns null for unknown datasource | 59 | not-applicable | — | — | Renovate's `go-get=1` meta tag parsing and datasource inference are not implemented in Rust. |
| returns null for go-import prefix mismatch | 72 | not-applicable | — | — | Renovate's `go-get=1` import-prefix mismatch validation is not implemented in Rust. |
| supports GitHub deps | 89 | not-applicable | — | — | Renovate's `go-get=1` GitHub tag datasource inference is not implemented in Rust. |
| supports GitHub EE deps | 104 | not-applicable | — | — | Renovate's hostRules-based GitHub Enterprise tag datasource inference is not implemented in Rust. |
| supports Go submodules in GitLab repo | 122 | not-applicable | — | — | Renovate's `go-get=1` GitLab submodule source inference is not implemented in Rust. |
| supports GitLab deps | 139 | not-applicable | — | — | Renovate's `go-get=1` GitLab tag datasource inference is not implemented in Rust. |
| supports GitLab deps on private subgroups | 156 | not-applicable | — | — | Renovate's GitLab private subgroup package-name normalization is not implemented in Rust. |
| does not fail for names containing .git | 173 | not-applicable | — | — | Renovate's GitLab package-name handling for `.git` inside path segments is not implemented in Rust. |
| supports GitLab with URL mismatch | 190 | not-applicable | — | — | Renovate's source URL based GitLab datasource inference is not implemented in Rust. |
| supports GitLab deps with version | 209 | not-applicable | — | — | Renovate's GitLab version-suffix package-name normalization is not implemented in Rust. |
| returns null for invalid GitLab EE go-source URL | 226 | not-applicable | — | — | Renovate's invalid self-hosted GitLab source URL handling is not implemented in Rust. |
| supports GitLab EE deps | 243 | not-applicable | — | — | Renovate's hostRules-based self-hosted GitLab tag datasource inference is not implemented in Rust. |
| supports GitLab EE deps in subgroup | 261 | not-applicable | — | — | Renovate's self-hosted GitLab subgroup package-name normalization is not implemented in Rust. |
| supports GitLab EE deps in private subgroup with api/ as part of packageName and api/v4 as part of endpoint | 279 | not-applicable | — | — | Renovate's self-hosted GitLab endpoint/package split logic is not implemented in Rust. |
| supports GitLab EE deps in subgroup with version | 302 | not-applicable | — | — | Renovate's self-hosted GitLab subgroup version-suffix normalization is not implemented in Rust. |
| supports GitLab EE deps in private subgroup with vcs indicator | 320 | not-applicable | — | — | Renovate's self-hosted GitLab `.git` VCS indicator stripping is not implemented in Rust. |
| supports GitLab EE deps in private subgroup with vcs indicator and subfolders | 338 | not-applicable | — | — | Renovate's self-hosted GitLab `.git` VCS indicator and subfolder handling is not implemented in Rust. |
| supports GitLab EE monorepo deps in subgroup | 356 | not-applicable | — | — | Renovate's self-hosted GitLab monorepo subgroup resolution is not implemented in Rust. |
| handles fyne.io | 374 | not-applicable | — | — | Renovate's `go-import` GitHub source inference from custom domains is not implemented in Rust. |
| handles fyne.io - go-import no quotes | 391 | not-applicable | — | — | Renovate's loose `go-import` HTML parser is not implemented in Rust. |
| handles go-import with gitlab source | 408 | not-applicable | — | — | Renovate's `go-import` GitLab source inference from custom domains is not implemented in Rust. |
| handles go-import with azure devops source | 427 | not-applicable | — | — | Renovate's `go-import` Azure DevOps git source inference is not implemented in Rust. |
| returns null for invalid azure devops source | 443 | not-applicable | — | — | Renovate's invalid Azure DevOps source handling is not implemented in Rust. |
| handles uncommon imports | 456 | not-applicable | — | — | Renovate's generic git source inference from `go-import` metadata is not implemented in Rust. |
| returns null for mod imports | 474 | not-applicable | — | — | Renovate's `go-import` VCS type filtering for `mod` imports is not implemented in Rust. |
| returns null for invalid import URL | 489 | not-applicable | — | — | Renovate's invalid `go-import` source URL handling is not implemented in Rust. |
| correctly splits a URL where the endpoint is contained | 504 | not-applicable | — | — | Renovate's self-hosted GitLab endpoint containment split logic is not implemented in Rust. |

---


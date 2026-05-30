# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/base.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/base.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/go/base › simple cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $module -> $datasource: $packageName | 17 | not-applicable | — | — | No corresponding Rust source|

### `modules/datasource/go/base › go-get requests › meta name=go-source`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown prefix | 46 | not-applicable | — | — | No corresponding Rust source|
| returns null for unknown datasource | 59 | not-applicable | — | — | No corresponding Rust source|
| returns null for go-import prefix mismatch | 72 | not-applicable | — | — | No corresponding Rust source|
| supports GitHub deps | 89 | not-applicable | — | — | No corresponding Rust source|
| supports GitHub EE deps | 104 | not-applicable | — | — | No corresponding Rust source|
| supports Go submodules in GitLab repo | 122 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab deps | 139 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab deps on private subgroups | 156 | not-applicable | — | — | No corresponding Rust source|
| does not fail for names containing .git | 173 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab with URL mismatch | 190 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab deps with version | 209 | not-applicable | — | — | No corresponding Rust source|
| returns null for invalid GitLab EE go-source URL | 226 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab EE deps | 243 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab EE deps in subgroup | 261 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab EE deps in private subgroup with api/ as part of packageName and api/v4 as part of endpoint | 279 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab EE deps in subgroup with version | 302 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab EE deps in private subgroup with vcs indicator | 320 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab EE deps in private subgroup with vcs indicator and subfolders | 338 | not-applicable | — | — | No corresponding Rust source|
| supports GitLab EE monorepo deps in subgroup | 356 | not-applicable | — | — | No corresponding Rust source|
| handles fyne.io | 374 | not-applicable | — | — | No corresponding Rust source|
| handles fyne.io - go-import no quotes | 391 | not-applicable | — | — | No corresponding Rust source|
| handles go-import with gitlab source | 408 | not-applicable | — | — | No corresponding Rust source|
| handles go-import with azure devops source | 427 | not-applicable | — | — | No corresponding Rust source|
| returns null for invalid azure devops source | 443 | not-applicable | — | — | No corresponding Rust source|
| handles uncommon imports | 456 | not-applicable | — | — | No corresponding Rust source|
| returns null for mod imports | 474 | not-applicable | — | — | No corresponding Rust source|
| returns null for invalid import URL | 489 | not-applicable | — | — | No corresponding Rust source|
| correctly splits a URL where the endpoint is contained | 504 | not-applicable | — | — | No corresponding Rust source|

---

# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/base.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/base.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** pending

### `modules/datasource/go/base › simple cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $module -> $datasource: $packageName | 17 | pending | — | — | —|

### `modules/datasource/go/base › go-get requests › meta name=go-source`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown prefix | 46 | pending | — | — | —|
| returns null for unknown datasource | 59 | pending | — | — | —|
| returns null for go-import prefix mismatch | 72 | pending | — | — | —|
| supports GitHub deps | 89 | pending | — | — | —|
| supports GitHub EE deps | 104 | pending | — | — | —|
| supports Go submodules in GitLab repo | 122 | pending | — | — | —|
| supports GitLab deps | 139 | pending | — | — | —|
| supports GitLab deps on private subgroups | 156 | pending | — | — | —|
| does not fail for names containing .git | 173 | pending | — | — | —|
| supports GitLab with URL mismatch | 190 | pending | — | — | —|
| supports GitLab deps with version | 209 | pending | — | — | —|
| returns null for invalid GitLab EE go-source URL | 226 | pending | — | — | —|
| supports GitLab EE deps | 243 | pending | — | — | —|
| supports GitLab EE deps in subgroup | 261 | pending | — | — | —|
| supports GitLab EE deps in private subgroup with api/ as part of packageName and api/v4 as part of endpoint | 279 | pending | — | — | —|
| supports GitLab EE deps in subgroup with version | 302 | pending | — | — | —|
| supports GitLab EE deps in private subgroup with vcs indicator | 320 | pending | — | — | —|
| supports GitLab EE deps in private subgroup with vcs indicator and subfolders | 338 | pending | — | — | —|
| supports GitLab EE monorepo deps in subgroup | 356 | pending | — | — | —|
| handles fyne.io | 374 | pending | — | — | —|
| handles fyne.io - go-import no quotes | 391 | pending | — | — | —|
| handles go-import with gitlab source | 408 | pending | — | — | —|
| handles go-import with azure devops source | 427 | pending | — | — | —|
| returns null for invalid azure devops source | 443 | pending | — | — | —|
| handles uncommon imports | 456 | pending | — | — | —|
| returns null for mod imports | 474 | pending | — | — | —|
| returns null for invalid import URL | 489 | pending | — | — | —|
| correctly splits a URL where the endpoint is contained | 504 | pending | — | — | —|

---

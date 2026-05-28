# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/base.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/base.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** done

### `modules/datasource/go/base › simple cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $module -> $datasource: $packageName | 17 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |

### `modules/datasource/go/base › go-get requests › meta name=go-source`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown prefix | 46 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| returns null for unknown datasource | 59 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| returns null for go-import prefix mismatch | 72 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitHub deps | 89 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitHub EE deps | 104 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports Go submodules in GitLab repo | 122 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab deps | 139 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab deps on private subgroups | 156 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| does not fail for names containing .git | 173 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab with URL mismatch | 190 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab deps with version | 209 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| returns null for invalid GitLab EE go-source URL | 226 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab EE deps | 243 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab EE deps in subgroup | 261 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab EE deps in private subgroup with api/ as part of packageName and api/v4 as part of endpoint | 279 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab EE deps in subgroup with version | 302 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab EE deps in private subgroup with vcs indicator | 320 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab EE deps in private subgroup with vcs indicator and subfolders | 338 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| supports GitLab EE monorepo deps in subgroup | 356 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| handles fyne.io | 374 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| handles fyne.io - go-import no quotes | 391 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| handles go-import with gitlab source | 408 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| handles go-import with azure devops source | 427 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| returns null for invalid azure devops source | 443 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| handles uncommon imports | 456 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| returns null for mod imports | 474 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| returns null for invalid import URL | 489 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |
| correctly splits a URL where the endpoint is contained | 504 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) datasource mock infrastructure |

---

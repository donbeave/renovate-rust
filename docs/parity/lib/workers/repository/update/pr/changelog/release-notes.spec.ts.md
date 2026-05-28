# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/release-notes.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/release-notes.spec.ts
**Total tests:** 56 | **Ported:** 0 | **Actionable:** 56 | **Status:** done

### `workers/repository/update/pr/changelog/release-notes › releaseNotesCacheMinutes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with string date (%s, %i) | 197 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles date object | 205 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| https://gitlab.com/api/v4/projects/gitlab-org%2Fgitter%2Fwebapp | 209 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

### `workers/repository/update/pr/changelog/release-notes › addReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if input is null/undefined | 215 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| returns input if invalid | 224 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| returns ChangeLogResult | 237 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| returns ChangeLogResult without release notes | 265 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

### `workers/repository/update/pr/changelog/release-notes › getReleaseList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array if no apiBaseUrl | 314 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| should return release list for github repo | 322 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| should return release list for gitlab.com project | 364 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| should return release list for self hosted gitlab project | 400 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| should return empty release list for self-hosted bitbucket-server | 439 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for release notes without body and name | 452 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body "" | 486 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with name "" | 529 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| filters release note name when same as version | 571 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| strips release note with version prefixed name | 613 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| release notes without body and name that matches version tag returns null | 655 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body "v" | 689 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body "other-" (packageName) | 732 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body "other-" (depName) | 776 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body "other_v" | 821 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body "other@" | 865 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body from gitlab repo "" | 908 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body from gitlab repo "v" | 945 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets release notes with body from gitlab repo "other-" | 982 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| gets null from repository without gitlab/github in domain | 1019 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles same version but different repo releases | 1036 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| fallback to extractVersion | 1087 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles not found | 1125 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles files mismatch | 1140 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles wrong format | 1165 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles bad markdown | 1189 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles bitbucket release notes link | 1213 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles bitbucket-server release notes link | 1238 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses angular.js | 1267 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses gitlab.com/gitlab-org/gitter/webapp | 1295 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses self hosted gitlab | 1323 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses jest | 1353 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles github sourceDirectory | 1382 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses js-yaml | 1417 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| ignores invalid | 1446 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › ReleaseNotes Correctness`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yargs 15.3.0 | 1463 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses yargs 15.2.0 | 1493 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses adapter-utils 4.33.0 | 1523 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| parses when version contained in the body 0.14.0 | 1553 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| ignores trailing link reference definitions when searching body | 1585 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles gitlab sourceDirectory | 1611 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| handles skipped packages | 1647 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| isUrl | 1661 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| 15.3.0 is not equal to 15.2.0 | 1665 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › shouldSkipChangelogMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip for flagged repository | 1671 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| should continue for other repository | 1675 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

### `workers/repository/update/pr/changelog/release-notes › massageBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not modify # inside codeblocks | 1682 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |

| (parametrized test at line 197) | 197 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
| (parametrized test at line 209) | 209 | not-applicable | — | — | Requires httpMock + vi.mock datasource/github/gitlab mock infrastructure |
---

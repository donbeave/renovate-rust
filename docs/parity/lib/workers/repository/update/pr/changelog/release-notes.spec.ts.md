# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/release-notes.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/release-notes.spec.ts
**Total tests:** 56 | **Ported:** 0 | **Actionable:** 56 | **Status:** pending

### `workers/repository/update/pr/changelog/release-notes › releaseNotesCacheMinutes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with string date (%s, %i) | 197 | pending | — | — | — |
| handles date object | 205 | pending | — | — | — |
| https://gitlab.com/api/v4/projects/gitlab-org%2Fgitter%2Fwebapp | 209 | pending | — | — | — |

### `workers/repository/update/pr/changelog/release-notes › addReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if input is null/undefined | 215 | pending | — | — | — |
| returns input if invalid | 224 | pending | — | — | — |
| returns ChangeLogResult | 237 | pending | — | — | — |
| returns ChangeLogResult without release notes | 265 | pending | — | — | — |

### `workers/repository/update/pr/changelog/release-notes › getReleaseList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array if no apiBaseUrl | 314 | pending | — | — | — |
| should return release list for github repo | 322 | pending | — | — | — |
| should return release list for gitlab.com project | 364 | pending | — | — | — |
| should return release list for self hosted gitlab project | 400 | pending | — | — | — |
| should return empty release list for self-hosted bitbucket-server | 439 | pending | — | — | — |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for release notes without body and name | 452 | pending | — | — | — |
| gets release notes with body "" | 486 | pending | — | — | — |
| gets release notes with name "" | 529 | pending | — | — | — |
| filters release note name when same as version | 571 | pending | — | — | — |
| strips release note with version prefixed name | 613 | pending | — | — | — |
| release notes without body and name that matches version tag returns null | 655 | pending | — | — | — |
| gets release notes with body "v" | 689 | pending | — | — | — |
| gets release notes with body "other-" (packageName) | 732 | pending | — | — | — |
| gets release notes with body "other-" (depName) | 776 | pending | — | — | — |
| gets release notes with body "other_v" | 821 | pending | — | — | — |
| gets release notes with body "other@" | 865 | pending | — | — | — |
| gets release notes with body from gitlab repo "" | 908 | pending | — | — | — |
| gets release notes with body from gitlab repo "v" | 945 | pending | — | — | — |
| gets release notes with body from gitlab repo "other-" | 982 | pending | — | — | — |
| gets null from repository without gitlab/github in domain | 1019 | pending | — | — | — |
| handles same version but different repo releases | 1036 | pending | — | — | — |
| fallback to extractVersion | 1087 | pending | — | — | — |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles not found | 1125 | pending | — | — | — |
| handles files mismatch | 1140 | pending | — | — | — |
| handles wrong format | 1165 | pending | — | — | — |
| handles bad markdown | 1189 | pending | — | — | — |
| handles bitbucket release notes link | 1213 | pending | — | — | — |
| handles bitbucket-server release notes link | 1238 | pending | — | — | — |
| parses angular.js | 1267 | pending | — | — | — |
| parses gitlab.com/gitlab-org/gitter/webapp | 1295 | pending | — | — | — |
| parses self hosted gitlab | 1323 | pending | — | — | — |
| parses jest | 1353 | pending | — | — | — |
| handles github sourceDirectory | 1382 | pending | — | — | — |
| parses js-yaml | 1417 | pending | — | — | — |
| ignores invalid | 1446 | pending | — | — | — |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › ReleaseNotes Correctness`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yargs 15.3.0 | 1463 | pending | — | — | — |
| parses yargs 15.2.0 | 1493 | pending | — | — | — |
| parses adapter-utils 4.33.0 | 1523 | pending | — | — | — |
| parses when version contained in the body 0.14.0 | 1553 | pending | — | — | — |
| ignores trailing link reference definitions when searching body | 1585 | pending | — | — | — |
| handles gitlab sourceDirectory | 1611 | pending | — | — | — |
| handles skipped packages | 1647 | pending | — | — | — |
| isUrl | 1661 | pending | — | — | — |
| 15.3.0 is not equal to 15.2.0 | 1665 | pending | — | — | — |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › shouldSkipChangelogMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip for flagged repository | 1671 | pending | — | — | — |
| should continue for other repository | 1675 | pending | — | — | — |

### `workers/repository/update/pr/changelog/release-notes › massageBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not modify # inside codeblocks | 1682 | pending | — | — | — |

| (parametrized test at line 197) | 197 | pending | — | — | — |
| (parametrized test at line 209) | 209 | pending | — | — | — |
---

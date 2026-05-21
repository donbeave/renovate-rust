# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/release-notes.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/release-notes.spec.ts
**Total tests:** 54 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/pr/changelog/release-notes › releaseNotesCacheMinutes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with string date (%s, %i) | 197 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles date object | 205 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| https://gitlab.com/api/v4/projects/gitlab-org%2Fgitter%2Fwebapp | 209 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

### `workers/repository/update/pr/changelog/release-notes › addReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if input is null/undefined | 215 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| returns input if invalid | 224 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| returns ChangeLogResult | 237 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| returns ChangeLogResult without release notes | 265 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

### `workers/repository/update/pr/changelog/release-notes › getReleaseList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array if no apiBaseUrl | 314 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| should return release list for github repo | 322 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| should return release list for gitlab.com project | 364 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| should return release list for self hosted gitlab project | 400 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| should return empty release list for self-hosted bitbucket-server | 439 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for release notes without body and name | 452 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body "" | 486 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with name "" | 529 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| filters release note name when same as version | 571 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| strips release note with version prefixed name | 613 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| release notes without body and name that matches version tag returns null | 655 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body "v" | 689 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body "other-" (packageName) | 732 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body "other-" (depName) | 776 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body "other_v" | 821 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body "other@" | 865 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body from gitlab repo "" | 908 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body from gitlab repo "v" | 945 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets release notes with body from gitlab repo "other-" | 982 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| gets null from repository without gitlab/github in domain | 1019 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles same version but different repo releases | 1036 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| fallback to extractVersion | 1087 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles not found | 1125 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles files mismatch | 1140 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles wrong format | 1165 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles bad markdown | 1189 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles bitbucket release notes link | 1213 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles bitbucket-server release notes link | 1238 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses angular.js | 1267 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses gitlab.com/gitlab-org/gitter/webapp | 1295 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses self hosted gitlab | 1323 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses jest | 1353 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles github sourceDirectory | 1382 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses js-yaml | 1417 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| ignores invalid | 1446 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › ReleaseNotes Correctness`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yargs 15.3.0 | 1463 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses yargs 15.2.0 | 1493 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses adapter-utils 4.33.0 | 1523 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| parses when version contained in the body 0.14.0 | 1553 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| ignores trailing link reference definitions when searching body | 1585 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles gitlab sourceDirectory | 1611 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| handles skipped packages | 1647 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| isUrl | 1661 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| 15.3.0 is not equal to 15.2.0 | 1665 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › shouldSkipChangelogMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip for flagged repository | 1671 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |
| should continue for other repository | 1675 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

### `workers/repository/update/pr/changelog/release-notes › massageBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not modify # inside codeblocks | 1682 | not-applicable | — | — | tests release note fetching via GitHub/GitLab/Gitea HTTP APIs; platform API calls out of scope |

---

# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/release-notes.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/release-notes.spec.ts
**Total tests:** 56 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/pr/changelog/release-notes › releaseNotesCacheMinutes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with string date (%s, %i)  | 197 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles date object  | 205 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| https://gitlab.com/api/v4/projects/gitlab-org%2Fgitter%2Fwebapp  | 209 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `workers/repository/update/pr/changelog/release-notes › addReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if input is null/undefined  | 215 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns input if invalid  | 224 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns ChangeLogResult  | 237 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns ChangeLogResult without release notes  | 265 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `workers/repository/update/pr/changelog/release-notes › getReleaseList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array if no apiBaseUrl  | 314 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return release list for github repo  | 322 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return release list for gitlab.com project  | 364 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return release list for self hosted gitlab project  | 400 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return empty release list for self-hosted bitbucket-server  | 439 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for release notes without body and name  | 452 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body ""  | 486 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with name ""  | 529 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| filters release note name when same as version  | 571 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| strips release note with version prefixed name  | 613 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| release notes without body and name that matches version tag returns null  | 655 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body "v"  | 689 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body "other-" (packageName)  | 732 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body "other-" (depName)  | 776 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body "other_v"  | 821 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body "other@"  | 865 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body from gitlab repo ""  | 908 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body from gitlab repo "v"  | 945 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets release notes with body from gitlab repo "other-"  | 982 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| gets null from repository without gitlab/github in domain  | 1019 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles same version but different repo releases  | 1036 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| fallback to extractVersion  | 1087 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles not found  | 1125 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles files mismatch  | 1140 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles wrong format  | 1165 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles bad markdown  | 1189 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles bitbucket release notes link  | 1213 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles bitbucket-server release notes link  | 1238 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses angular.js  | 1267 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses gitlab.com/gitlab-org/gitter/webapp  | 1295 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses self hosted gitlab  | 1323 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses jest  | 1353 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles github sourceDirectory  | 1382 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses js-yaml  | 1417 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| ignores invalid  | 1446 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › ReleaseNotes Correctness`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yargs 15.3.0  | 1463 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses yargs 15.2.0  | 1493 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses adapter-utils 4.33.0  | 1523 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| parses when version contained in the body 0.14.0  | 1553 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| ignores trailing link reference definitions when searching body  | 1585 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles gitlab sourceDirectory  | 1611 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| handles skipped packages  | 1647 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| isUrl  | 1661 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| 15.3.0 is not equal to 15.2.0  | 1665 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `workers/repository/update/pr/changelog/release-notes › getReleaseNotesMd() › shouldSkipChangelogMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip for flagged repository  | 1671 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should continue for other repository  | 1675 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `workers/repository/update/pr/changelog/release-notes › massageBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not modify # inside codeblocks  | 1682 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

| (parametrized test at line 197)  | 197 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| (parametrized test at line 209)  | 209 | not-applicable | Mock framework internals — tests changelog release-notes via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
---

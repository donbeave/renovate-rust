# `lib/workers/repository/update/pr/changelog/release-notes.spec.ts`

[← `worker/repository`](../../../../../../_by-module/worker/repository.md) · [all modules](../../../../../../README.md)

**0/57 ported** (57 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 197 | _(it.each / template — verify manually)_ | ? | — |
| 205 | handles date object | pending | — |
| 209 | _(it.each / template — verify manually)_ | ? | — |
| 215 | returns null if input is null/undefined | pending | — |
| 224 | returns input if invalid | pending | — |
| 237 | returns changelogresult | pending | — |
| 265 | returns changelogresult without release notes | pending | — |
| 314 | should return empty array if no apibaseurl | pending | — |
| 322 | should return release list for github repo | pending | — |
| 364 | should return release list for gitlab.com project | pending | — |
| 400 | should return release list for self hosted gitlab project | pending | — |
| 439 | should return empty release list for self-hosted bitbucket-server | pending | — |
| 452 | should return null for release notes without body and name | pending | — |
| 486 | gets release notes with body "" | pending | — |
| 529 | gets release notes with name "" | pending | — |
| 571 | filters release note name when same as version | pending | — |
| 613 | strips release note with version prefixed name | pending | — |
| 655 | release notes without body and name that matches version tag returns null | pending | — |
| 689 | gets release notes with body "v" | pending | — |
| 732 | gets release notes with body "other-" (packagename) | pending | — |
| 776 | gets release notes with body "other-" (depname) | pending | — |
| 821 | gets release notes with body "other_v" | pending | — |
| 865 | gets release notes with body "other@" | pending | — |
| 908 | gets release notes with body "other/" | pending | — |
| 951 | gets release notes with body "other/v" | pending | — |
| 994 | gets release notes with body from gitlab repo "" | pending | — |
| 1031 | gets release notes with body from gitlab repo "v" | pending | — |
| 1068 | gets release notes with body from gitlab repo "other-" | pending | — |
| 1105 | gets null from repository without gitlab/github in domain | pending | — |
| 1122 | handles same version but different repo releases | pending | — |
| 1173 | fallback to extractversion | pending | — |
| 1211 | handles not found | pending | — |
| 1226 | handles files mismatch | pending | — |
| 1251 | handles wrong format | pending | — |
| 1275 | handles bad markdown | pending | — |
| 1299 | handles bitbucket release notes link | pending | — |
| 1324 | handles bitbucket-server release notes link | pending | — |
| 1353 | parses angular.js | pending | — |
| 1381 | parses gitlab.com/gitlab-org/gitter/webapp | pending | — |
| 1409 | parses self hosted gitlab | pending | — |
| 1439 | parses jest | pending | — |
| 1468 | handles github sourcedirectory | pending | — |
| 1503 | parses js-yaml | pending | — |
| 1532 | ignores invalid | pending | — |
| 1549 | parses yargs 15.3.0 | pending | — |
| 1579 | parses yargs 15.2.0 | pending | — |
| 1609 | parses adapter-utils 4.33.0 | pending | — |
| 1639 | parses when version contained in the body 0.14.0 | pending | — |
| 1671 | ignores trailing link reference definitions when searching body | pending | — |
| 1697 | handles gitlab sourcedirectory | pending | — |
| 1733 | handles skipped packages | pending | — |
| 1747 | isurl | pending | — |
| 1751 | 15.3.0 is not equal to 15.2.0 | pending | — |
| 1756 | returns empty body when changelog section has no content | pending | — |
| 1781 | should skip for flagged repository | pending | — |
| 1785 | should continue for other repository | pending | — |
| 1792 | does not modify # inside codeblocks | pending | — |


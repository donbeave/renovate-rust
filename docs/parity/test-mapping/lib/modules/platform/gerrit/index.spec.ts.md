# `lib/modules/platform/gerrit/index.spec.ts`

[← `platform/gerrit`](../../../../_by-module/platform/gerrit.md) · [all modules](../../../../README.md)

**0/63 ported** (63 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 59 | should throw if no endpoint | pending | — |
| 64 | should throw if no username/password | pending | — |
| 71 | should init | pending | — |
| 81 | should throw if auth fails | pending | — |
| 92 | should throw if version is unparseable | pending | — |
| 105 | returns repos | pending | — |
| 111 | initrepo() - inactive | pending | — |
| 133 | initrepo() - active | pending | — |
| 146 | initrepo() - passes clonesubmodules | pending | — |
| 163 | initrepo() - abandon rejected changes | pending | — |
| 193 | findpr() - no results | pending | — |
| 214 | findpr() - found | pending | — |
| 237 | getpr() - found | pending | — |
| 251 | getpr() - not found | pending | — |
| 256 | getpr() - other error | pending | — |
| 267 | updatepr() - closed => abandon the change | pending | — |
| 278 | updatepr() - body set => add as message if needed | pending | — |
| 295 | updatepr() - with addlabels => add hashtags | pending | — |
| 308 | updatepr() - with removelabels => remove hashtags | pending | — |
| 321 | updatepr() - with addlabels and removelabels => update hashtags in single call | pending | — |
| 337 | updatepr() - targetbranch set => move the change | pending | — |
| 355 | createpr() - creates change by pushing to refs/for/ | pending | — |
| 388 | createpr() - with autoapprove | pending | — |
| 424 | createpr() - with labels | pending | — |
| 463 | createpr() - no change found after push => rejects | pending | — |
| 478 | createpr() - push fails => rejects | pending | — |
| 494 | getbranchpr() - no result | pending | — |
| 509 | getbranchpr() - found | pending | — |
| 535 | getbranchpr() - found even without targetbranch | pending | — |
| 563 | getprlist() - empty list | pending | — |
| 575 | getprlist() - multiple results | pending | — |
| 591 | mergepr() - blocker by verified | pending | — |
| 600 | mergepr() - success | pending | — |
| 607 | mergepr() - other errors | pending | — |
| 616 | getbranchstatus() - change not found => yellow | pending | — |
| 623 | getbranchstatus() - change found, submittable and not hasproblems => green | pending | — |
| 633 | getbranchstatus() - change found, submittable but hasproblems => red | pending | — |
| 650 | getbranchstatus() - change found and hasproblems => red | pending | — |
| 667 | getbranchstatus() - changes found and hasblockinglabels but no problems => red | pending | — |
| 694 | _(it.each / template — verify manually)_ | ? | — |
| 718 | _(it.each / template — verify manually)_ | ? | — |
| 766 | setbranchstatus(renovate/stability-days) | pending | — |
| 778 | setbranchstatus(renovate/merge-confidence) | pending | — |
| 803 | _(it.each / template — verify manually)_ | ? | — |
| 855 | no change found | pending | — |
| 868 | does not call setlabel() if label does not exist in change | pending | — |
| 893 | deletelabel() - deletes a label | pending | — |
| 903 | addreviewers() - add reviewers | pending | — |
| 916 | addassignees() - set assignee | pending | — |
| 929 | ensurecomment() - without tag | pending | — |
| 942 | ensurecomment() - with tag | pending | — |
| 961 | getrawfile() - repo and branch | pending | — |
| 972 | getrawfile() - repo/branch from config | pending | — |
| 986 | getrawfile() - branch defaults | pending | — |
| 1000 | getrawfile() - no repo | pending | — |
| 1013 | getjsonfile() | pending | — |
| 1022 | massagemarkdown() | pending | — |
| 1053 | deletelabel() | pending | — |
| 1059 | ensurecommentremoval() | pending | — |
| 1069 | ensureissueclosing() | pending | — |
| 1073 | ensureissue() | pending | — |
| 1079 | findissue() | pending | — |
| 1083 | getissuelist() | pending | — |


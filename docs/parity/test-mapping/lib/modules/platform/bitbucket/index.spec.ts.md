# `lib/modules/platform/bitbucket/index.spec.ts`

[← `platform/bitbucket`](../../../../_by-module/platform/bitbucket.md) · [all modules](../../../../README.md)

**0/96 in-scope tests ported** (96 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 68 | should throw if no token or username/password | pending | — |
| 73 | should show warning message if custom endpoint | pending | — |
| 85 | should init with username/password | pending | — |
| 99 | should init with only token | pending | — |
| 112 | should warn for missing "profile" scope | pending | — |
| 126 | returns repos | pending | — |
| 160 | uses configured namespaces directly without fetching workspaces | pending | — |
| 177 | filters repos based on autodiscoverprojects patterns | pending | — |
| 205 | filters repos based on autodiscoverprojects patterns with negation | pending | — |
| 235 | works with username and password | pending | — |
| 255 | works with only api token | pending | — |
| 279 | works with only access token | pending | — |
| 305 | not enabled: defaults to using main branch | pending | — |
| 325 | enabled: uses development branch when development branch exists | pending | — |
| 352 | enabled: falls back to mainbranch if development branch does not exist | pending | — |
| 378 | bitbucket finds pr for branch | pending | — |
| 390 | returns null if no pr for branch | pending | — |
| 403 | getbranchstatus 3 | pending | — |
| 425 | getbranchstatus 4 | pending | — |
| 450 | getbranchstatus 5 | pending | — |
| 477 | getbranchstatus 6 | pending | — |
| 501 | getbranchstatus 7 | pending | — |
| 549 | getbranchstatuscheck 1 | pending | — |
| 553 | getbranchstatuscheck 2 | pending | — |
| 557 | getbranchstatuscheck 3 | pending | — |
| 563 | posts status | pending | — |
| 589 | does not throw | pending | — |
| 616 | returns null if no issues | pending | — |
| 635 | updates existing issues | pending | — |
| 666 | creates new issue | pending | — |
| 691 | noop for existing issue | pending | — |
| 725 | does not throw for disabled issues | pending | — |
| 730 | closes issue | pending | — |
| 761 | returns empty array for disabled issues | pending | — |
| 766 | get issues | pending | — |
| 797 | does not throw | pending | — |
| 812 | does not throw | pending | — |
| 818 | should add the given reviewers to the pr | pending | — |
| 830 | should handle reviewers as username or uuid | pending | — |
| 853 | does not throw | pending | — |
| 869 | does not throw | pending | — |
| 885 | exists | pending | — |
| 889 | filters pr list by author | pending | — |
| 913 | exists | pending | — |
| 917 | finds pr | pending | — |
| 931 | finds closed pr with no reopen comments | pending | — |
| 968 | finds closed pr with reopen comment on private repository | pending | — |
| 1005 | finds closed pr with reopen comment on public repository from workspace member | pending | — |
| 1048 | finds closed pr with reopen comment on public repository from non-workspace member | pending | — |
| 1091 | finds pr from other authors | pending | — |
| 1113 | returns null if no open pr exists - (includeotherauthors) | pending | — |
| 1133 | posts pr | pending | — |
| 1179 | removes inactive reviewers when creating pr | pending | — |
| 1262 | removes default reviewers no longer member of the workspace when creating pr | pending | — |
| 1325 | throws exception when unable to check default reviewers workspace membership | pending | — |
| 1373 | removes reviewer if they are also the author of the pr | pending | — |
| 1428 | rethrows exception when pr create error due to unknown reviewers error | pending | — |
| 1469 | rethrows exception when pr create error not due to reviewers field | pending | — |
| 1510 | lists pr tasks and resolves the unresolved tasks | pending | — |
| 1584 | swallows list pr error and pr creation succeeds | pending | — |
| 1613 | swallows resolve pr task error and pr creation succeeds | pending | — |
| 1663 | exists | pending | — |
| 1669 | canrebase | pending | — |
| 1692 | reviewers | pending | — |
| 1719 | removes html tags | pending | — |
| 1728 | updates pull request url links | pending | — |
| 1736 | updates issues url links | pending | — |
| 1744 | dependency dashboard: updates abandoned dependencies heading and place note inside | pending | — |
| 1761 | dependency dashboard: updates vulnerabilities section with multiple collapsible details sections to nested list | pending | — |
| 1786 | dependency dashboard: updates detected dependencies section with multiple collapsible details sections to nested list | pending | — |
| 1812 | updates release notes section | pending | — |
| 1830 | updates codeblocks to correct indentation level | pending | — |
| 1851 | updates codeblocks to drop extra language data | pending | — |
| 1874 | puts pr | pending | — |
| 1900 | removes inactive reviewers when updating pr | pending | — |
| 1968 | removes reviewers no longer member of the workspace when updating pr | pending | — |
| 2017 | throws exception when unable to check reviewers workspace membership | pending | — |
| 2051 | rethrows exception when pr update error due to unknown reviewers error | pending | — |
| 2076 | rethrows exception when pr create error not due to reviewers field | pending | — |
| 2103 | throws an error on failure to get current list of reviewers | pending | — |
| 2113 | closes pr | pending | — |
| 2139 | pr cache gets updated after a pr is created | pending | — |
| 2202 | pr cache gets updated after a pr is updated | pending | — |
| 2246 | posts merge with optional merge strategy | pending | — |
| 2257 | posts merge with auto | pending | — |
| 2269 | posts merge with merge-commit | pending | — |
| 2281 | posts merge with squash | pending | — |
| 2293 | does not post merge with rebase | pending | — |
| 2302 | posts merge with fast-forward | pending | — |
| 2316 | returns file content | pending | — |
| 2326 | returns file content in json5 format | pending | — |
| 2341 | returns file content from given repo | pending | — |
| 2351 | returns file content from branch or tag | pending | — |
| 2361 | returns file content from branch with a slash in its name | pending | — |
| 2378 | throws on malformed json | pending | — |
| 2386 | throws on errors | pending | — |


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket/index.spec.ts
**Total tests:** 96 | **Ported:** 0 | **Actionable:** 96 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token or username/password | 68 | pending | — | — | — |
| should show warning message if custom endpoint | 73 | pending | — | — | — |
| should init with username/password | 85 | pending | — | — | — |
| should init with only token | 99 | pending | — | — | — |
| should warn for missing "profile" scope | 112 | pending | — | — | — |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 126 | pending | — | — | — |
| uses configured namespaces directly without fetching workspaces | 160 | pending | — | — | — |
| filters repos based on autodiscoverProjects patterns | 177 | pending | — | — | — |
| filters repos based on autodiscoverProjects patterns with negation | 205 | pending | — | — | — |

### `initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with username and password | 235 | pending | — | — | — |
| works with only API token | 255 | pending | — | — | — |
| works with only access token | 279 | pending | — | — | — |

### `bbUseDevelopmentBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| not enabled: defaults to using main branch | 305 | pending | — | — | — |
| enabled: uses development branch when development branch exists | 325 | pending | — | — | — |
| enabled: falls back to mainbranch if development branch does not exist | 352 | pending | — | — | — |

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bitbucket finds PR for branch | 378 | pending | — | — | — |
| returns null if no PR for branch | 390 | pending | — | — | — |

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchStatus 3 | 403 | pending | — | — | — |
| getBranchStatus 4 | 425 | pending | — | — | — |
| getBranchStatus 5 | 450 | pending | — | — | — |
| getBranchStatus 6 | 477 | pending | — | — | — |
| getBranchStatus 7 | 501 | pending | — | — | — |

### `getBranchStatusCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchStatusCheck 1 | 549 | pending | — | — | — |
| getBranchStatusCheck 2 | 553 | pending | — | — | — |
| getBranchStatusCheck 3 | 557 | pending | — | — | — |

### `setBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts status | 563 | pending | — | — | — |

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 589 | pending | — | — | — |
| returns null if no issues | 616 | pending | — | — | — |

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates existing issues | 635 | pending | — | — | — |
| creates new issue | 666 | pending | — | — | — |
| noop for existing issue | 691 | pending | — | — | — |

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw for disabled issues | 725 | pending | — | — | — |
| closes issue | 730 | pending | — | — | — |

### `getIssueList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array for disabled issues | 761 | pending | — | — | — |
| get issues | 766 | pending | — | — | — |
| does not throw | 797 | pending | — | — | — |

### `addAssignees()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 812 | pending | — | — | — |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given reviewers to the PR | 818 | pending | — | — | — |
| should handle reviewers as username or UUID | 830 | pending | — | — | — |

### `ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 853 | pending | — | — | — |

### `ensureCommentRemoval()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 869 | pending | — | — | — |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| exists | 885 | pending | — | — | — |
| filters PR list by author | 889 | pending | — | — | — |

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| exists | 913 | pending | — | — | — |
| finds pr | 917 | pending | — | — | — |
| finds closed pr with no reopen comments | 931 | pending | — | — | — |
| finds closed pr with reopen comment on private repository | 968 | pending | — | — | — |
| finds closed pr with reopen comment on public repository from workspace member | 1005 | pending | — | — | — |
| finds closed pr with reopen comment on public repository from non-workspace member | 1048 | pending | — | — | — |
| finds pr from other authors | 1091 | pending | — | — | — |
| returns null if no open pr exists - (includeOtherAuthors) | 1113 | pending | — | — | — |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 1133 | pending | — | — | — |
| removes inactive reviewers when creating pr | 1179 | pending | — | — | — |
| removes default reviewers no longer member of the workspace when creating pr | 1262 | pending | — | — | — |
| throws exception when unable to check default reviewers workspace membership | 1325 | pending | — | — | — |
| removes reviewer if they are also the author of the pr | 1373 | pending | — | — | — |
| rethrows exception when PR create error due to unknown reviewers error | 1428 | pending | — | — | — |
| rethrows exception when PR create error not due to reviewers field | 1469 | pending | — | — | — |
| lists PR tasks and resolves the unresolved tasks | 1510 | pending | — | — | — |
| swallows list PR error and PR creation succeeds | 1584 | pending | — | — | — |
| swallows resolve PR task error and PR creation succeeds | 1613 | pending | — | — | — |

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| exists | 1663 | pending | — | — | — |
| canRebase | 1669 | pending | — | — | — |
| reviewers | 1692 | pending | — | — | — |

### `massageMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes html tags | 1719 | pending | — | — | — |
| updates pull request url links | 1728 | pending | — | — | — |
| updates issues url links | 1736 | pending | — | — | — |
| dependency dashboard: updates abandoned dependencies heading and place note inside | 1744 | pending | — | — | — |
| dependency dashboard: updates vulnerabilities section with multiple collapsible details sections to nested list | 1761 | pending | — | — | — |
| dependency dashboard: updates detected dependencies section with multiple collapsible details sections to nested list | 1786 | pending | — | — | — |
| updates release notes section | 1812 | pending | — | — | — |
| updates codeblocks to correct indentation level | 1830 | pending | — | — | — |
| updates codeblocks to drop extra language data | 1851 | pending | — | — | — |

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| puts PR | 1874 | pending | — | — | — |
| removes inactive reviewers when updating pr | 1900 | pending | — | — | — |
| removes reviewers no longer member of the workspace when updating pr | 1968 | pending | — | — | — |
| throws exception when unable to check reviewers workspace membership | 2017 | pending | — | — | — |
| rethrows exception when PR update error due to unknown reviewers error | 2051 | pending | — | — | — |
| rethrows exception when PR create error not due to reviewers field | 2076 | pending | — | — | — |
| throws an error on failure to get current list of reviewers | 2103 | pending | — | — | — |
| closes PR | 2113 | pending | — | — | — |

### `maintains pr cache integrity at runtime`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| pr cache gets updated after a pr is created | 2139 | pending | — | — | — |
| pr cache gets updated after a pr is updated | 2202 | pending | — | — | — |

### `mergePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts Merge with optional merge strategy | 2246 | pending | — | — | — |
| posts Merge with auto | 2257 | pending | — | — | — |
| posts Merge with merge-commit | 2269 | pending | — | — | — |
| posts Merge with squash | 2281 | pending | — | — | — |
| does not post Merge with rebase | 2293 | pending | — | — | — |
| posts Merge with fast-forward | 2302 | pending | — | — | — |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 2316 | pending | — | — | — |
| returns file content in json5 format | 2326 | pending | — | — | — |
| returns file content from given repo | 2341 | pending | — | — | — |
| returns file content from branch or tag | 2351 | pending | — | — | — |
| returns file content from branch with a slash in its name | 2361 | pending | — | — | — |
| throws on malformed JSON | 2378 | pending | — | — | — |
| throws on errors | 2386 | pending | — | — | — |

---


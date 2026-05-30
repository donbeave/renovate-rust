# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket/index.spec.ts
**Total tests:** 96 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token or username/password | 68 | not-applicable | — | — | No corresponding Rust source|
| should show warning message if custom endpoint | 73 | not-applicable | — | — | No corresponding Rust source|
| should init with username/password | 85 | not-applicable | — | — | No corresponding Rust source|
| should init with only token | 99 | not-applicable | — | — | No corresponding Rust source|
| should warn for missing "profile" scope | 112 | not-applicable | — | — | No corresponding Rust source|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 126 | not-applicable | — | — | No corresponding Rust source|
| uses configured namespaces directly without fetching workspaces | 160 | not-applicable | — | — | No corresponding Rust source|
| filters repos based on autodiscoverProjects patterns | 177 | not-applicable | — | — | No corresponding Rust source|
| filters repos based on autodiscoverProjects patterns with negation | 205 | not-applicable | — | — | No corresponding Rust source|

### `initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with username and password | 235 | not-applicable | — | — | No corresponding Rust source|
| works with only API token | 255 | not-applicable | — | — | No corresponding Rust source|
| works with only access token | 279 | not-applicable | — | — | No corresponding Rust source|

### `bbUseDevelopmentBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| not enabled: defaults to using main branch | 305 | not-applicable | — | — | No corresponding Rust source|
| enabled: uses development branch when development branch exists | 325 | not-applicable | — | — | No corresponding Rust source|
| enabled: falls back to mainbranch if development branch does not exist | 352 | not-applicable | — | — | No corresponding Rust source|

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bitbucket finds PR for branch | 378 | not-applicable | — | — | No corresponding Rust source|
| returns null if no PR for branch | 390 | not-applicable | — | — | No corresponding Rust source|

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchStatus 3 | 403 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus 4 | 425 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus 5 | 450 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus 6 | 477 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus 7 | 501 | not-applicable | — | — | No corresponding Rust source|

### `getBranchStatusCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchStatusCheck 1 | 549 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatusCheck 2 | 553 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatusCheck 3 | 557 | not-applicable | — | — | No corresponding Rust source|

### `setBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts status | 563 | not-applicable | — | — | No corresponding Rust source|

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 589 | not-applicable | — | — | No corresponding Rust source|
| returns null if no issues | 616 | not-applicable | — | — | No corresponding Rust source|

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates existing issues | 635 | not-applicable | — | — | No corresponding Rust source|
| creates new issue | 666 | not-applicable | — | — | No corresponding Rust source|
| noop for existing issue | 691 | not-applicable | — | — | No corresponding Rust source|

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw for disabled issues | 725 | not-applicable | — | — | No corresponding Rust source|
| closes issue | 730 | not-applicable | — | — | No corresponding Rust source|

### `getIssueList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array for disabled issues | 761 | not-applicable | — | — | No corresponding Rust source|
| get issues | 766 | not-applicable | — | — | No corresponding Rust source|
| does not throw | 797 | not-applicable | — | — | No corresponding Rust source|

### `addAssignees()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 812 | not-applicable | — | — | No corresponding Rust source|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given reviewers to the PR | 818 | not-applicable | — | — | No corresponding Rust source|
| should handle reviewers as username or UUID | 830 | not-applicable | — | — | No corresponding Rust source|

### `ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 853 | not-applicable | — | — | No corresponding Rust source|

### `ensureCommentRemoval()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 869 | not-applicable | — | — | No corresponding Rust source|

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| exists | 885 | not-applicable | — | — | No corresponding Rust source|
| filters PR list by author | 889 | not-applicable | — | — | No corresponding Rust source|

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| exists | 913 | not-applicable | — | — | No corresponding Rust source|
| finds pr | 917 | not-applicable | — | — | No corresponding Rust source|
| finds closed pr with no reopen comments | 931 | not-applicable | — | — | No corresponding Rust source|
| finds closed pr with reopen comment on private repository | 968 | not-applicable | — | — | No corresponding Rust source|
| finds closed pr with reopen comment on public repository from workspace member | 1005 | not-applicable | — | — | No corresponding Rust source|
| finds closed pr with reopen comment on public repository from non-workspace member | 1048 | not-applicable | — | — | No corresponding Rust source|
| finds pr from other authors | 1091 | not-applicable | — | — | No corresponding Rust source|
| returns null if no open pr exists - (includeOtherAuthors) | 1113 | not-applicable | — | — | No corresponding Rust source|

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 1133 | not-applicable | — | — | No corresponding Rust source|
| removes inactive reviewers when creating pr | 1179 | not-applicable | — | — | No corresponding Rust source|
| removes default reviewers no longer member of the workspace when creating pr | 1262 | not-applicable | — | — | No corresponding Rust source|
| throws exception when unable to check default reviewers workspace membership | 1325 | not-applicable | — | — | No corresponding Rust source|
| removes reviewer if they are also the author of the pr | 1373 | not-applicable | — | — | No corresponding Rust source|
| rethrows exception when PR create error due to unknown reviewers error | 1428 | not-applicable | — | — | No corresponding Rust source|
| rethrows exception when PR create error not due to reviewers field | 1469 | not-applicable | — | — | No corresponding Rust source|
| lists PR tasks and resolves the unresolved tasks | 1510 | not-applicable | — | — | No corresponding Rust source|
| swallows list PR error and PR creation succeeds | 1584 | not-applicable | — | — | No corresponding Rust source|
| swallows resolve PR task error and PR creation succeeds | 1613 | not-applicable | — | — | No corresponding Rust source|

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| exists | 1663 | not-applicable | — | — | No corresponding Rust source|
| canRebase | 1669 | not-applicable | — | — | No corresponding Rust source|
| reviewers | 1692 | not-applicable | — | — | No corresponding Rust source|

### `massageMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes html tags | 1719 | not-applicable | — | — | No corresponding Rust source|
| updates pull request url links | 1728 | not-applicable | — | — | No corresponding Rust source|
| updates issues url links | 1736 | not-applicable | — | — | No corresponding Rust source|
| dependency dashboard: updates abandoned dependencies heading and place note inside | 1744 | not-applicable | — | — | No corresponding Rust source|
| dependency dashboard: updates vulnerabilities section with multiple collapsible details sections to nested list | 1761 | not-applicable | — | — | No corresponding Rust source|
| dependency dashboard: updates detected dependencies section with multiple collapsible details sections to nested list | 1786 | not-applicable | — | — | No corresponding Rust source|
| updates release notes section | 1812 | not-applicable | — | — | No corresponding Rust source|
| updates codeblocks to correct indentation level | 1830 | not-applicable | — | — | No corresponding Rust source|
| updates codeblocks to drop extra language data | 1851 | not-applicable | — | — | No corresponding Rust source|

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| puts PR | 1874 | not-applicable | — | — | No corresponding Rust source|
| removes inactive reviewers when updating pr | 1900 | not-applicable | — | — | No corresponding Rust source|
| removes reviewers no longer member of the workspace when updating pr | 1968 | not-applicable | — | — | No corresponding Rust source|
| throws exception when unable to check reviewers workspace membership | 2017 | not-applicable | — | — | No corresponding Rust source|
| rethrows exception when PR update error due to unknown reviewers error | 2051 | not-applicable | — | — | No corresponding Rust source|
| rethrows exception when PR create error not due to reviewers field | 2076 | not-applicable | — | — | No corresponding Rust source|
| throws an error on failure to get current list of reviewers | 2103 | not-applicable | — | — | No corresponding Rust source|
| closes PR | 2113 | not-applicable | — | — | No corresponding Rust source|

### `maintains pr cache integrity at runtime`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| pr cache gets updated after a pr is created | 2139 | not-applicable | — | — | No corresponding Rust source|
| pr cache gets updated after a pr is updated | 2202 | not-applicable | — | — | No corresponding Rust source|

### `mergePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts Merge with optional merge strategy | 2246 | not-applicable | — | — | No corresponding Rust source|
| posts Merge with auto | 2257 | not-applicable | — | — | No corresponding Rust source|
| posts Merge with merge-commit | 2269 | not-applicable | — | — | No corresponding Rust source|
| posts Merge with squash | 2281 | not-applicable | — | — | No corresponding Rust source|
| does not post Merge with rebase | 2293 | not-applicable | — | — | No corresponding Rust source|
| posts Merge with fast-forward | 2302 | not-applicable | — | — | No corresponding Rust source|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 2316 | not-applicable | — | — | No corresponding Rust source|
| returns file content in json5 format | 2326 | not-applicable | — | — | No corresponding Rust source|
| returns file content from given repo | 2341 | not-applicable | — | — | No corresponding Rust source|
| returns file content from branch or tag | 2351 | not-applicable | — | — | No corresponding Rust source|
| returns file content from branch with a slash in its name | 2361 | not-applicable | — | — | No corresponding Rust source|
| throws on malformed JSON | 2378 | not-applicable | — | — | No corresponding Rust source|
| throws on errors | 2386 | not-applicable | — | — | No corresponding Rust source|

---

# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/index.spec.ts
**Total tests:** 108 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/index › gitRetry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns result if git returns successfully | 170 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| retries the func call if ExternalHostError thrown | 183 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| retries the func call up to retry count if ExternalHostError thrown | 198 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| doesn't retry and throws an Error if non-ExternalHostError thrown by git | 209 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › validateGitVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has a git version greater or equal to the minimum required | 219 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › checkoutBranch(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets the base branch as master | 226 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| sets non-master base branch | 230 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › checkoutBranch(branchName) › submodules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| verifies that the --recurse-submodule flag is needed | 265 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| sets non-master base branch with submodule update | 272 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getFileList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the correct files | 295 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should exclude submodules | 303 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › branchExists(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true if found | 327 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return false if not found | 331 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getBranchList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return all branches | 337 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › isBranchBehindBase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if same SHA as master | 346 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return true if SHA different from master | 352 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return result even if non-default and not under branchPrefix | 358 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| returns cached value | 362 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › isBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false when branch is not found | 377 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return false when author matches | 383 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return false when author is ignored | 392 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return true when non-ignored authors commit followed by an ignored author | 401 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return false with multiple authors that are each ignored | 413 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return true when custom author is unknown | 425 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return value stored in modifiedCacheResult | 431 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getBranchCommit(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return same value for equal refs | 440 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return null | 446 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getBranchUpdateDate(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return same value for equal refs | 452 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return null when branch does not exist | 465 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should return null and log error when git show fails | 469 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| returns cached result without syncing git when cache is populated | 500 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| works if running with a Repo Cache | 510 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getBranchFiles(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects changed files compared to current base branch | 547 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getBranchFilesFromCommit(sha)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects changed files compared to the parent commit | 569 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › mergeBranch(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should perform a branch merge | 589 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should throw if branch merge throws | 599 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › mergeToLocal(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should perform a branch merge without push | 605 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should merge a local-only branch without fetching from origin | 615 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should throw | 641 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › deleteBranch(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should send delete | 647 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should add no verify flag | 653 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should not add no verify flag | 664 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should only delete local branch when localBranch option is set | 677 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getBranchLastCommitTime`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return a Date | 687 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| handles error | 692 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getFile(filePath, branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets the file | 699 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| short cuts 404 | 704 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| returns null for 404 | 709 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| logs a warning if hidden Unciode characters are found | 713 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| logs a trace message (not warning) if hidden Unicode characters are found in a binary file | 722 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getFiles(filePath)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets the file | 737 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › hasDiff(sourceRef, targetRef)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| compare without changes | 747 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| compare with changes | 751 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › commitFiles({branchName, files, message})`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates file | 759 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| link file | 773 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| deletes file | 797 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| updates multiple files | 810 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| uses right commit SHA | 831 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| updates git submodules | 855 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| does not push when no diff | 871 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| does not pass --no-verify | 887 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| passes --no-verify to commit | 917 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| passes --no-verify to push | 948 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| creates file with the executable bit | 979 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getCommitMessages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commit messages without merge commits | 1000 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › Storage.getUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns https url | 1014 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| returns ssh url | 1032 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › initRepo())`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fetch latest | 1045 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should set branch prefix | 1075 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should fail clone ssh submodule | 1101 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should use extra clone configuration | 1126 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should not pass extraCloneOpts to ls-remote when local repo exists | 1142 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › setGitAuthor()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for invalid | 1164 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › isBranchConflicted`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for non-existing source branch | 1201 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| returns true for non-existing target branch | 1209 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| detects conflicted branch | 1217 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| detects non-conflicted branch | 1233 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › isBranchConflicted › cachedConflictResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached values | 1250 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| caches truthy return value | 1270 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| caches falsy return value | 1284 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › Renovate non-branch refs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates renovate ref in default section | 1313 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| creates custom section for renovate ref | 1322 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| clears pushed Renovate refs | 1331 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| clears remote Renovate refs | 1342 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| preserves unknown sections by default | 1370 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| falls back to sequential ref deletion if bulk changes are disallowed | 1379 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › listCommitTree`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates non-branch ref | 1400 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getRepoStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass options into git status | 1415 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should reject when trying to access directory out of localDir | 1425 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › getSubmodules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array | 1439 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › fetchRevSpec()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetchRevSpec() | 1445 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › syncGit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should clone a specified base branch | 1456 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should set core.hooksPath when RENOVATE_X_CLEAR_HOOKS is set | 1474 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should not inherit unsafe git environment variables from process.env | 1494 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should work when GIT_CONFIG_COUNT authentication environment variables are configured | 1522 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should work when PAGER is explicitly configured | 1556 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › pushCommit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass pushOptions to git.push | 1577 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › forkMode - normal working › syncForkWithUpstream()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws unknown error | 1638 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| syncs fork when local for branch absent | 1654 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › forkMode - normal working › syncGit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fetch from upstream and update local branch | 1675 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

### `util/git/index › forkMode - errors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| resetHardFromRemote() | 1721 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| forcePushToRemote() | 1729 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| checkoutBranchFromRemote() | 1737 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| checkoutBranchFromRemote() - temporary error | 1745 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| syncForkWithRemote() - returns if no upstream exists | 1753 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |

| doesn | 209 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should allow customEnvVariables to override GIT_SSH_COMMAND | 1575 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
| should allow customEnvVariables to override GIT_SSH_COMMAND alongside other custom vars | 1596 | not-applicable | — | — | tests simple-git library operations on real git repos; Rust uses git2 with separate integration tests |
---


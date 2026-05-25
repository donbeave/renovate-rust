# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/index.spec.ts
**Total tests:** 108 | **Ported:** 0 | **Actionable:** 108 | **Status:** pending

### `util/git/index › gitRetry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns result if git returns successfully | 170 | pending | — | — | — |
| retries the func call if ExternalHostError thrown | 183 | pending | — | — | — |
| retries the func call up to retry count if ExternalHostError thrown | 198 | pending | — | — | — |
| doesn't retry and throws an Error if non-ExternalHostError thrown by git | 209 | pending | — | — | — |

### `util/git/index › validateGitVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has a git version greater or equal to the minimum required | 219 | pending | — | — | — |

### `util/git/index › checkoutBranch(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets the base branch as master | 226 | pending | — | — | — |
| sets non-master base branch | 230 | pending | — | — | — |

### `util/git/index › checkoutBranch(branchName) › submodules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| verifies that the --recurse-submodule flag is needed | 265 | pending | — | — | — |
| sets non-master base branch with submodule update | 272 | pending | — | — | — |

### `util/git/index › getFileList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the correct files | 295 | pending | — | — | — |
| should exclude submodules | 303 | pending | — | — | — |

### `util/git/index › branchExists(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true if found | 327 | pending | — | — | — |
| should return false if not found | 331 | pending | — | — | — |

### `util/git/index › getBranchList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return all branches | 337 | pending | — | — | — |

### `util/git/index › isBranchBehindBase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if same SHA as master | 346 | pending | — | — | — |
| should return true if SHA different from master | 352 | pending | — | — | — |
| should return result even if non-default and not under branchPrefix | 358 | pending | — | — | — |
| returns cached value | 362 | pending | — | — | — |

### `util/git/index › isBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false when branch is not found | 377 | pending | — | — | — |
| should return false when author matches | 383 | pending | — | — | — |
| should return false when author is ignored | 392 | pending | — | — | — |
| should return true when non-ignored authors commit followed by an ignored author | 401 | pending | — | — | — |
| should return false with multiple authors that are each ignored | 413 | pending | — | — | — |
| should return true when custom author is unknown | 425 | pending | — | — | — |
| should return value stored in modifiedCacheResult | 431 | pending | — | — | — |

### `util/git/index › getBranchCommit(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return same value for equal refs | 440 | pending | — | — | — |
| should return null | 446 | pending | — | — | — |

### `util/git/index › getBranchUpdateDate(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return same value for equal refs | 452 | pending | — | — | — |
| should return null when branch does not exist | 465 | pending | — | — | — |
| should return null and log error when git show fails | 469 | pending | — | — | — |
| returns cached result without syncing git when cache is populated | 500 | pending | — | — | — |
| works if running with a Repo Cache | 510 | pending | — | — | — |

### `util/git/index › getBranchFiles(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects changed files compared to current base branch | 547 | pending | — | — | — |

### `util/git/index › getBranchFilesFromCommit(sha)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects changed files compared to the parent commit | 569 | pending | — | — | — |

### `util/git/index › mergeBranch(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should perform a branch merge | 589 | pending | — | — | — |
| should throw if branch merge throws | 599 | pending | — | — | — |

### `util/git/index › mergeToLocal(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should perform a branch merge without push | 605 | pending | — | — | — |
| should merge a local-only branch without fetching from origin | 615 | pending | — | — | — |
| should throw | 641 | pending | — | — | — |

### `util/git/index › deleteBranch(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should send delete | 647 | pending | — | — | — |
| should add no verify flag | 653 | pending | — | — | — |
| should not add no verify flag | 664 | pending | — | — | — |
| should only delete local branch when localBranch option is set | 677 | pending | — | — | — |

### `util/git/index › getBranchLastCommitTime`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return a Date | 687 | pending | — | — | — |
| handles error | 692 | pending | — | — | — |

### `util/git/index › getFile(filePath, branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets the file | 699 | pending | — | — | — |
| short cuts 404 | 704 | pending | — | — | — |
| returns null for 404 | 709 | pending | — | — | — |
| logs a warning if hidden Unciode characters are found | 713 | pending | — | — | — |
| logs a trace message (not warning) if hidden Unicode characters are found in a binary file | 722 | pending | — | — | — |

### `util/git/index › getFiles(filePath)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets the file | 737 | pending | — | — | — |

### `util/git/index › hasDiff(sourceRef, targetRef)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| compare without changes | 747 | pending | — | — | — |
| compare with changes | 751 | pending | — | — | — |

### `util/git/index › commitFiles({branchName, files, message})`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates file | 759 | pending | — | — | — |
| link file | 773 | pending | — | — | — |
| deletes file | 797 | pending | — | — | — |
| updates multiple files | 810 | pending | — | — | — |
| uses right commit SHA | 831 | pending | — | — | — |
| updates git submodules | 855 | pending | — | — | — |
| does not push when no diff | 871 | pending | — | — | — |
| does not pass --no-verify | 887 | pending | — | — | — |
| passes --no-verify to commit | 917 | pending | — | — | — |
| passes --no-verify to push | 948 | pending | — | — | — |
| creates file with the executable bit | 979 | pending | — | — | — |

### `util/git/index › getCommitMessages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commit messages without merge commits | 1000 | pending | — | — | — |

### `util/git/index › Storage.getUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns https url | 1014 | pending | — | — | — |
| returns ssh url | 1032 | pending | — | — | — |

### `util/git/index › initRepo())`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fetch latest | 1045 | pending | — | — | — |
| should set branch prefix | 1075 | pending | — | — | — |
| should fail clone ssh submodule | 1101 | pending | — | — | — |
| should use extra clone configuration | 1126 | pending | — | — | — |
| should not pass extraCloneOpts to ls-remote when local repo exists | 1142 | pending | — | — | — |

### `util/git/index › setGitAuthor()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for invalid | 1164 | pending | — | — | — |

### `util/git/index › isBranchConflicted`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for non-existing source branch | 1201 | pending | — | — | — |
| returns true for non-existing target branch | 1209 | pending | — | — | — |
| detects conflicted branch | 1217 | pending | — | — | — |
| detects non-conflicted branch | 1233 | pending | — | — | — |

### `util/git/index › isBranchConflicted › cachedConflictResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached values | 1250 | pending | — | — | — |
| caches truthy return value | 1270 | pending | — | — | — |
| caches falsy return value | 1284 | pending | — | — | — |

### `util/git/index › Renovate non-branch refs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates renovate ref in default section | 1313 | pending | — | — | — |
| creates custom section for renovate ref | 1322 | pending | — | — | — |
| clears pushed Renovate refs | 1331 | pending | — | — | — |
| clears remote Renovate refs | 1342 | pending | — | — | — |
| preserves unknown sections by default | 1370 | pending | — | — | — |
| falls back to sequential ref deletion if bulk changes are disallowed | 1379 | pending | — | — | — |

### `util/git/index › listCommitTree`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates non-branch ref | 1400 | pending | — | — | — |

### `util/git/index › getRepoStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass options into git status | 1415 | pending | — | — | — |
| should reject when trying to access directory out of localDir | 1425 | pending | — | — | — |

### `util/git/index › getSubmodules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array | 1439 | pending | — | — | — |

### `util/git/index › fetchRevSpec()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetchRevSpec() | 1445 | pending | — | — | — |

### `util/git/index › syncGit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should clone a specified base branch | 1456 | pending | — | — | — |
| should set core.hooksPath when RENOVATE_X_CLEAR_HOOKS is set | 1474 | pending | — | — | — |
| should not inherit unsafe git environment variables from process.env | 1494 | pending | — | — | — |
| should work when GIT_CONFIG_COUNT authentication environment variables are configured | 1522 | pending | — | — | — |
| should work when PAGER is explicitly configured | 1556 | pending | — | — | — |

### `util/git/index › pushCommit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass pushOptions to git.push | 1577 | pending | — | — | — |

### `util/git/index › forkMode - normal working › syncForkWithUpstream()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws unknown error | 1638 | pending | — | — | — |
| syncs fork when local for branch absent | 1654 | pending | — | — | — |

### `util/git/index › forkMode - normal working › syncGit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fetch from upstream and update local branch | 1675 | pending | — | — | — |

### `util/git/index › forkMode - errors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| resetHardFromRemote() | 1721 | pending | — | — | — |
| forcePushToRemote() | 1729 | pending | — | — | — |
| checkoutBranchFromRemote() | 1737 | pending | — | — | — |
| checkoutBranchFromRemote() - temporary error | 1745 | pending | — | — | — |
| syncForkWithRemote() - returns if no upstream exists | 1753 | pending | — | — | — |

| doesn | 209 | pending | — | — | — |
| should allow customEnvVariables to override GIT_SSH_COMMAND | 1575 | pending | — | — | — |
| should allow customEnvVariables to override GIT_SSH_COMMAND alongside other custom vars | 1596 | pending | — | — | — |
---


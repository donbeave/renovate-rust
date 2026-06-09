# `lib/util/git/index.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**1/107 in-scope tests ported** (106 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 170 | returns result if git returns successfully | pending | — |
| 183 | retries the func call if externalhosterror thrown | pending | — |
| 198 | retries the func call up to retry count if externalhosterror thrown | pending | — |
| 209 | doesn't retry and throws an error if non-externalhosterror thrown by git | pending | — |
| 219 | has a git version greater or equal to the minimum required | ported | [`crates/renovate-core/src/workers/global/initialize.rs:444`](../../../../../../crates/renovate-core/src/workers/global/initialize.rs#L444) |
| 226 | sets the base branch as master | pending | — |
| 230 | sets non-master base branch | pending | — |
| 265 | verifies that the --recurse-submodule flag is needed | pending | — |
| 272 | sets non-master base branch with submodule update | pending | — |
| 295 | should return the correct files | pending | — |
| 303 | should exclude submodules | pending | — |
| 327 | should return true if found | pending | — |
| 331 | should return false if not found | pending | — |
| 337 | should return all branches | pending | — |
| 346 | should return false if same sha as master | pending | — |
| 352 | should return true if sha different from master | pending | — |
| 358 | should return result even if non-default and not under branchprefix | pending | — |
| 362 | returns cached value | pending | — |
| 377 | should return false when branch is not found | pending | — |
| 383 | should return false when author matches | pending | — |
| 392 | should return false when author is ignored | pending | — |
| 401 | should return true when non-ignored authors commit followed by an ignored author | pending | — |
| 413 | should return false with multiple authors that are each ignored | pending | — |
| 425 | should return true when custom author is unknown | pending | — |
| 431 | should return value stored in modifiedcacheresult | pending | — |
| 440 | should return same value for equal refs | pending | — |
| 446 | should return null | pending | — |
| 452 | should return same value for equal refs | pending | — |
| 465 | should return null when branch does not exist | pending | — |
| 469 | should return null and log error when git show fails | pending | — |
| 500 | returns cached result without syncing git when cache is populated | pending | — |
| 510 | works if running with a repo cache | pending | — |
| 547 | detects changed files compared to current base branch | pending | — |
| 569 | detects changed files compared to the parent commit | pending | — |
| 589 | should perform a branch merge | pending | — |
| 599 | should throw if branch merge throws | pending | — |
| 605 | should perform a branch merge without push | pending | — |
| 615 | should merge a local-only branch without fetching from origin | pending | — |
| 641 | should throw | pending | — |
| 647 | should send delete | pending | — |
| 653 | should add no verify flag | pending | — |
| 664 | should not add no verify flag | pending | — |
| 677 | should only delete local branch when localbranch option is set | pending | — |
| 687 | should return a date | pending | — |
| 692 | handles error | pending | — |
| 699 | gets the file | pending | — |
| 704 | short cuts 404 | pending | — |
| 709 | returns null for 404 | pending | — |
| 713 | logs a warning if hidden unciode characters are found | pending | — |
| 722 | logs a trace message (not warning) if hidden unicode characters are found in a binary file | pending | — |
| 737 | gets the file | pending | — |
| 747 | compare without changes | pending | — |
| 751 | compare with changes | pending | — |
| 759 | creates file | pending | — |
| 773 | link file | pending | — |
| 797 | deletes file | pending | — |
| 810 | updates multiple files | pending | — |
| 831 | uses right commit sha | pending | — |
| 855 | updates git submodules | pending | — |
| 871 | does not push when no diff | pending | — |
| 887 | does not pass --no-verify | pending | — |
| 917 | passes --no-verify to commit | pending | — |
| 948 | passes --no-verify to push | pending | — |
| 979 | creates file with the executable bit | pending | — |
| 1000 | returns commit messages without merge commits | pending | — |
| 1014 | returns https url | pending | — |
| 1032 | returns ssh url | pending | — |
| 1045 | should fetch latest | pending | — |
| 1075 | should set branch prefix | pending | — |
| 1101 | should fail clone ssh submodule | pending | — |
| 1126 | should use extra clone configuration | pending | — |
| 1142 | should not pass extracloneopts to ls-remote when local repo exists | pending | — |
| 1164 | throws for invalid | pending | — |
| 1201 | returns true for non-existing source branch | pending | — |
| 1209 | returns true for non-existing target branch | pending | — |
| 1217 | detects conflicted branch | pending | — |
| 1233 | detects non-conflicted branch | pending | — |
| 1250 | returns cached values | pending | — |
| 1270 | caches truthy return value | pending | — |
| 1284 | caches falsy return value | pending | — |
| 1313 | creates renovate ref in default section | pending | — |
| 1322 | creates custom section for renovate ref | pending | — |
| 1331 | clears pushed renovate refs | pending | — |
| 1342 | clears remote renovate refs | pending | — |
| 1370 | preserves unknown sections by default | pending | — |
| 1379 | falls back to sequential ref deletion if bulk changes are disallowed | pending | — |
| 1400 | creates non-branch ref | pending | — |
| 1415 | should pass options into git status | pending | — |
| 1425 | should reject when trying to access directory out of localdir | pending | — |
| 1439 | should return empty array | pending | — |
| 1445 | fetchrevspec() | pending | — |
| 1456 | should clone a specified base branch | pending | — |
| 1474 | should set core.hookspath when renovate_x_clear_hooks is set | pending | — |
| 1494 | should not inherit unsafe git environment variables from process.env | pending | — |
| 1522 | should work when git_config_count authentication environment variables are configured | pending | — |
| 1556 | should allow customenvvariables to override git_ssh_command | pending | — |
| 1577 | should allow customenvvariables to override git_ssh_command alongside other custom vars | pending | — |
| 1598 | should allow process.env git_ssh_command to override the default | pending | — |
| 1615 | should pass pushoptions to git.push | pending | — |
| 1676 | throws unknown error | pending | — |
| 1692 | syncs fork when local for branch absent | pending | — |
| 1713 | should fetch from upstream and update local branch | pending | — |
| 1759 | resethardfromremote() | pending | — |
| 1767 | forcepushtoremote() | pending | — |
| 1775 | checkoutbranchfromremote() | pending | — |
| 1783 | checkoutbranchfromremote() - temporary error | pending | — |
| 1791 | syncforkwithremote() - returns if no upstream exists | pending | — |


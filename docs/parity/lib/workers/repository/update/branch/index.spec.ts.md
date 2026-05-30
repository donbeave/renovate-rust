# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/index.spec.ts
**Total tests:** 101 | **Ported:** 0 | **Actionable:** 101 | **Status:** pending

### `workers/repository/update/branch/index › processBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips branch if not scheduled and branch does not exist  | 157 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch creation if minimumGroupSize is not met  | 167 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if not scheduled and not updating out of schedule  | 180 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch for fresh release with minimumReleaseAge  | 198 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if minimumReleaseAge not met  | 223 | pending | — | — | branch worker runtime behavior is in scope |

### `workers/repository/update/branch/index › processBranch › if release is missing releaseTimestamp with minimumReleaseAge set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips branch if minimumReleaseAgeBehaviour=timestamp-required  | 241 | pending | — | — | branch worker runtime behavior is in scope |
| does not skip branch if minimumReleaseAgeBehaviour=timestamp-optional  | 260 | pending | — | — | branch worker runtime behavior is in scope |
| does not skip branch if minimumReleaseAgeBehaviour=timestamp-required and minimumReleaseAge=0 days  | 275 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if minimumConfidence not met  | 291 | pending | — | — | branch worker runtime behavior is in scope |
| processes branch if minimumConfidence is met  | 310 | pending | — | — | branch worker runtime behavior is in scope |
| processes branch if not scheduled but updating out of schedule  | 329 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if closed major PR found  | 343 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if closed digest PR found  | 358 | pending | — | — | branch worker runtime behavior is in scope |
| allows branch but disables automerge if merged PR found  | 373 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if closed minor PR found  | 388 | pending | — | — | branch worker runtime behavior is in scope |
| allows branch even if merged PR found  | 402 | pending | — | — | branch worker runtime behavior is in scope |
| throws error if closed PR found  | 418 | pending | — | — | branch worker runtime behavior is in scope |
| does not skip branch if edited PR found with rebaseLabel  | 432 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if edited PR found  | 451 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if tagretBranch of update PR is changed by user  | 478 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if edited PR found without commenting  | 510 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if target branch changed  | 534 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if branch edited and no PR found  | 570 | pending | — | — | branch worker runtime behavior is in scope |
| continues branch if branch edited and but PR found  | 581 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch if branch edited and and PR found with sha mismatch  | 595 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch creation limit exceeded  | 607 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch does not exist and in silent mode  | 624 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch needs dependencyDashboardApproval  | 642 | pending | — | — | branch worker runtime behavior is in scope |
| returns if pr creation limit exceeded and branch exists  | 660 | pending | — | — | branch worker runtime behavior is in scope |
| returns if commits per run limit exceeded  | 683 | pending | — | — | branch worker runtime behavior is in scope |
| does not return if commits per run limit exceeded but rebase requested  | 707 | pending | — | — | branch worker runtime behavior is in scope |
| returns if commits hourly limit exceeded  | 727 | pending | — | — | branch worker runtime behavior is in scope |
| does not return if commits hourly limit exceeded but rebase requested  | 746 | pending | — | — | branch worker runtime behavior is in scope |
| returns if no work  | 768 | pending | — | — | branch worker runtime behavior is in scope |
| returns if pending checks  | 785 | pending | — | — | branch worker runtime behavior is in scope |
| returns if pending checks - but branch exists  | 801 | pending | — | — | branch worker runtime behavior is in scope |
| automerges when there is no pr and, pr-creation is off-schedule  | 826 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch automerged  | 847 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch automerged and no checks  | 865 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch automerged (dry-run)  | 886 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch exists and prCreation set to approval  | 906 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch exists but pending  | 931 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch automerge is pending  | 957 | pending | — | — | branch worker runtime behavior is in scope |
| returns if PR creation failed  | 983 | pending | — | — | branch worker runtime behavior is in scope |
| handles unknown PrBlockedBy  | 1009 | pending | — | — | branch worker runtime behavior is in scope |
| retries setting branch status checks after PR creation  | 1035 | pending | — | — | branch worker runtime behavior is in scope |
| does not retry setting branch status checks when PR is not created  | 1067 | pending | — | — | branch worker runtime behavior is in scope |
| returns if branch exists but updated  | 1099 | pending | — | — | branch worker runtime behavior is in scope |
| updates branch when no fingerprint match  | 1130 | pending | — | — | branch worker runtime behavior is in scope |
| updates branch when forceRebase=true  | 1162 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and comments notice  | 1194 | pending | — | — | branch worker runtime behavior is in scope |
| fetches changelogs for the "branch" stage  | 1228 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and tries automerge  | 1247 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR when impossible to automerge  | 1271 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR when impossible to automerge with mismatch keepUpdatedLabel  | 1299 | pending | — | — | branch worker runtime behavior is in scope |
| skips when automerge is off schedule  | 1328 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR when impossible to automerge because off schedule  | 1364 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and adds lock file error comment with default message if no releaseTimestamp  | 1392 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and adds lock file error comment with user configured message if no releaseTimestamp  | 1422 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and adds lock file error comment with templated user configured message if no releaseTimestamp  | 1458 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and adds lock file error comment if old releaseTimestamp  | 1495 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and adds lock file error comment if new releaseTimestamp and branch exists  | 1520 | pending | — | — | branch worker runtime behavior is in scope |
| throws error if lock file errors and new releaseTimestamp  | 1545 | pending | — | — | branch worker runtime behavior is in scope |
| ensures PR and adds lock file error comment recreate closed  | 1568 | pending | — | — | branch worker runtime behavior is in scope |
| swallows branch errors  | 1593 | pending | — | — | branch worker runtime behavior is in scope |
| throws and swallows branch errors  | 1606 | pending | — | — | branch worker runtime behavior is in scope |
| rebases branch onto new basebranch if baseBranch changed by user  | 1627 | pending | — | — | branch worker runtime behavior is in scope |
| rebases branch onto new basebranch if no fingerprint found  | 1660 | pending | — | — | branch worker runtime behavior is in scope |
| rebases branch onto new basebranch if no fingerprint found - 2  | 1700 | pending | — | — | branch worker runtime behavior is in scope |
| swallows pr errors  | 1735 | pending | — | — | branch worker runtime behavior is in scope |
| closed pr (dry run)  | 1761 | pending | — | — | branch worker runtime behavior is in scope |
| branch pr no rebase (dry run)  | 1776 | pending | — | — | branch worker runtime behavior is in scope |
| branch pr no schedule lockfile (dry run)  | 1797 | pending | — | — | branch worker runtime behavior is in scope |
| branch pr no schedule (dry run)  | 1842 | pending | — | — | branch worker runtime behavior is in scope |
| branch pr no schedule  | 1890 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch update if stopUpdatingLabel presents  | 1932 | pending | — | — | branch worker runtime behavior is in scope |
| skips branch update if same updates  | 1968 | pending | — | — | branch worker runtime behavior is in scope |
| updates branch if stopUpdatingLabel presents and PR rebase/retry box checked  | 1996 | pending | — | — | branch worker runtime behavior is in scope |
| updates branch if stopUpdatingLabel presents and dependency dashboard box checked  | 2038 | pending | — | — | branch worker runtime behavior is in scope |
| executes post-upgrade tasks if trust is high  | 2077 | pending | — | — | branch worker runtime behavior is in scope |
| handles post-upgrade task exec errors  | 2176 | pending | — | — | branch worker runtime behavior is in scope |
| executes post-upgrade tasks with disabled post-upgrade command templating  | 2260 | pending | — | — | branch worker runtime behavior is in scope |
| executes post-upgrade tasks with multiple dependecy in one branch  | 2354 | pending | — | — | branch worker runtime behavior is in scope |
| executes post-upgrade tasks once when set to branch mode  | 2521 | pending | — | — | branch worker runtime behavior is in scope |
| executes post-upgrade tasks with propagated post-upgrade file path via env variable  | 2650 | pending | — | — | branch worker runtime behavior is in scope |
| should not propagate post-upgrade file path via env variable if the post-upgrade file creation failed  | 2755 | pending | — | — | branch worker runtime behavior is in scope |
| returns when rebaseWhen=never  | 2862 | pending | — | — | branch worker runtime behavior is in scope |
| continues when rebaseWhen=never and keepUpdatedLabel  | 2878 | pending | — | — | branch worker runtime behavior is in scope |
| returns when rebaseWhen=never and keepUpdatedLabel does not match  | 2905 | pending | — | — | branch worker runtime behavior is in scope |
| continues when rebaseWhen=never but checked  | 2932 | pending | — | — | branch worker runtime behavior is in scope |
| continues when checked by checkedBranches  | 2952 | pending | — | — | branch worker runtime behavior is in scope |
| does nothing when branchPrefixOld/branch and its pr exists  | 2973 | pending | — | — | branch worker runtime behavior is in scope |
| does nothing when branchPrefixOld/branch and its pr exists but updates not necessary  | 3012 | pending | — | — | branch worker runtime behavior is in scope |
| Dependency Dashboard All Pending approval  | 3051 | pending | — | — | branch worker runtime behavior is in scope |
| Dependency Dashboard open all rate-limited  | 3088 | pending | — | — | branch worker runtime behavior is in scope |
| Dependency Dashboard open all awaiting schedule  | 3125 | pending | — | — | branch worker runtime behavior is in scope |
| continues branch, skips automerge if there are artifact errors  | 3162 | pending | — | — | branch worker runtime behavior is in scope |
| continues to update PR, if branch got updated, even when prCreation!==immediate  | 3189 | pending | — | — | branch worker runtime behavior is in scope |
| checks out baseBranch after committing files  | 3222 | pending | — | — | branch worker runtime behavior is in scope |
| should not reattempt platform automerge without commitSha  | 3244 | pending | — | — | branch worker runtime behavior is in scope |
| should not reattempt platform automerge in dry run  | 3268 | pending | — | — | branch worker runtime behavior is in scope |

---

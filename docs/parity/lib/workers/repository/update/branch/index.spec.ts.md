# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/index.spec.ts
**Total tests:** 101 | **Ported:** 0 | **Actionable:** 101 | **Status:** pending

### `workers/repository/update/branch/index › processBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips branch if not scheduled and branch does not exist | 157 | pending | — | — | —|
| skips branch creation if minimumGroupSize is not met | 167 | pending | — | — | —|
| skips branch if not scheduled and not updating out of schedule | 180 | pending | — | — | —|
| skips branch for fresh release with minimumReleaseAge | 198 | pending | — | — | —|
| skips branch if minimumReleaseAge not met | 223 | pending | — | — | —|

### `workers/repository/update/branch/index › processBranch › if release is missing releaseTimestamp with minimumReleaseAge set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips branch if minimumReleaseAgeBehaviour=timestamp-required | 241 | pending | — | — | —|
| does not skip branch if minimumReleaseAgeBehaviour=timestamp-optional | 260 | pending | — | — | —|
| does not skip branch if minimumReleaseAgeBehaviour=timestamp-required and minimumReleaseAge=0 days | 275 | pending | — | — | —|
| skips branch if minimumConfidence not met | 291 | pending | — | — | —|
| processes branch if minimumConfidence is met | 310 | pending | — | — | —|
| processes branch if not scheduled but updating out of schedule | 329 | pending | — | — | —|
| skips branch if closed major PR found | 343 | pending | — | — | —|
| skips branch if closed digest PR found | 358 | pending | — | — | —|
| allows branch but disables automerge if merged PR found | 373 | pending | — | — | —|
| skips branch if closed minor PR found | 388 | pending | — | — | —|
| allows branch even if merged PR found | 402 | pending | — | — | —|
| throws error if closed PR found | 418 | pending | — | — | —|
| does not skip branch if edited PR found with rebaseLabel | 432 | pending | — | — | —|
| skips branch if edited PR found | 451 | pending | — | — | —|
| skips branch if tagretBranch of update PR is changed by user | 478 | pending | — | — | —|
| skips branch if edited PR found without commenting | 510 | pending | — | — | —|
| skips branch if target branch changed | 534 | pending | — | — | —|
| skips branch if branch edited and no PR found | 570 | pending | — | — | —|
| continues branch if branch edited and but PR found | 581 | pending | — | — | —|
| skips branch if branch edited and and PR found with sha mismatch | 595 | pending | — | — | —|
| returns if branch creation limit exceeded | 607 | pending | — | — | —|
| returns if branch does not exist and in silent mode | 624 | pending | — | — | —|
| returns if branch needs dependencyDashboardApproval | 642 | pending | — | — | —|
| returns if pr creation limit exceeded and branch exists | 660 | pending | — | — | —|
| returns if commits per run limit exceeded | 683 | pending | — | — | —|
| does not return if commits per run limit exceeded but rebase requested | 707 | pending | — | — | —|
| returns if commits hourly limit exceeded | 727 | pending | — | — | —|
| does not return if commits hourly limit exceeded but rebase requested | 746 | pending | — | — | —|
| returns if no work | 768 | pending | — | — | —|
| returns if pending checks | 785 | pending | — | — | —|
| returns if pending checks - but branch exists | 801 | pending | — | — | —|
| automerges when there is no pr and, pr-creation is off-schedule | 826 | pending | — | — | —|
| returns if branch automerged | 847 | pending | — | — | —|
| returns if branch automerged and no checks | 865 | pending | — | — | —|
| returns if branch automerged (dry-run) | 886 | pending | — | — | —|
| returns if branch exists and prCreation set to approval | 906 | pending | — | — | —|
| returns if branch exists but pending | 931 | pending | — | — | —|
| returns if branch automerge is pending | 957 | pending | — | — | —|
| returns if PR creation failed | 983 | pending | — | — | —|
| handles unknown PrBlockedBy | 1009 | pending | — | — | —|
| retries setting branch status checks after PR creation | 1035 | pending | — | — | —|
| does not retry setting branch status checks when PR is not created | 1067 | pending | — | — | —|
| returns if branch exists but updated | 1099 | pending | — | — | —|
| updates branch when no fingerprint match | 1130 | pending | — | — | —|
| updates branch when forceRebase=true | 1162 | pending | — | — | —|
| ensures PR and comments notice | 1194 | pending | — | — | —|
| fetches changelogs for the "branch" stage | 1228 | pending | — | — | —|
| ensures PR and tries automerge | 1247 | pending | — | — | —|
| ensures PR when impossible to automerge | 1271 | pending | — | — | —|
| ensures PR when impossible to automerge with mismatch keepUpdatedLabel | 1299 | pending | — | — | —|
| skips when automerge is off schedule | 1328 | pending | — | — | —|
| ensures PR when impossible to automerge because off schedule | 1364 | pending | — | — | —|
| ensures PR and adds lock file error comment with default message if no releaseTimestamp | 1392 | pending | — | — | —|
| ensures PR and adds lock file error comment with user configured message if no releaseTimestamp | 1422 | pending | — | — | —|
| ensures PR and adds lock file error comment with templated user configured message if no releaseTimestamp | 1458 | pending | — | — | —|
| ensures PR and adds lock file error comment if old releaseTimestamp | 1495 | pending | — | — | —|
| ensures PR and adds lock file error comment if new releaseTimestamp and branch exists | 1520 | pending | — | — | —|
| throws error if lock file errors and new releaseTimestamp | 1545 | pending | — | — | —|
| ensures PR and adds lock file error comment recreate closed | 1568 | pending | — | — | —|
| swallows branch errors | 1593 | pending | — | — | —|
| throws and swallows branch errors | 1606 | pending | — | — | —|
| rebases branch onto new basebranch if baseBranch changed by user | 1627 | pending | — | — | —|
| rebases branch onto new basebranch if no fingerprint found | 1660 | pending | — | — | —|
| rebases branch onto new basebranch if no fingerprint found - 2 | 1700 | pending | — | — | —|
| swallows pr errors | 1735 | pending | — | — | —|
| closed pr (dry run) | 1761 | pending | — | — | —|
| branch pr no rebase (dry run) | 1776 | pending | — | — | —|
| branch pr no schedule lockfile (dry run) | 1797 | pending | — | — | —|
| branch pr no schedule (dry run) | 1842 | pending | — | — | —|
| branch pr no schedule | 1890 | pending | — | — | —|
| skips branch update if stopUpdatingLabel presents | 1932 | pending | — | — | —|
| skips branch update if same updates | 1968 | pending | — | — | —|
| updates branch if stopUpdatingLabel presents and PR rebase/retry box checked | 1996 | pending | — | — | —|
| updates branch if stopUpdatingLabel presents and dependency dashboard box checked | 2038 | pending | — | — | —|
| executes post-upgrade tasks if trust is high | 2077 | pending | — | — | —|
| handles post-upgrade task exec errors | 2176 | pending | — | — | —|
| executes post-upgrade tasks with disabled post-upgrade command templating | 2260 | pending | — | — | —|
| executes post-upgrade tasks with multiple dependecy in one branch | 2354 | pending | — | — | —|
| executes post-upgrade tasks once when set to branch mode | 2521 | pending | — | — | —|
| executes post-upgrade tasks with propagated post-upgrade file path via env variable | 2650 | pending | — | — | —|
| should not propagate post-upgrade file path via env variable if the post-upgrade file creation failed | 2755 | pending | — | — | —|
| returns when rebaseWhen=never | 2862 | pending | — | — | —|
| continues when rebaseWhen=never and keepUpdatedLabel | 2878 | pending | — | — | —|
| returns when rebaseWhen=never and keepUpdatedLabel does not match | 2905 | pending | — | — | —|
| continues when rebaseWhen=never but checked | 2932 | pending | — | — | —|
| continues when checked by checkedBranches | 2952 | pending | — | — | —|
| does nothing when branchPrefixOld/branch and its pr exists | 2973 | pending | — | — | —|
| does nothing when branchPrefixOld/branch and its pr exists but updates not necessary | 3012 | pending | — | — | —|
| Dependency Dashboard All Pending approval | 3051 | pending | — | — | —|
| Dependency Dashboard open all rate-limited | 3088 | pending | — | — | —|
| Dependency Dashboard open all awaiting schedule | 3125 | pending | — | — | —|
| continues branch, skips automerge if there are artifact errors | 3162 | pending | — | — | —|
| continues to update PR, if branch got updated, even when prCreation!==immediate | 3189 | pending | — | — | —|
| checks out baseBranch after committing files | 3222 | pending | — | — | —|
| should not reattempt platform automerge without commitSha | 3244 | pending | — | — | —|
| should not reattempt platform automerge in dry run | 3268 | pending | — | — | —|

---

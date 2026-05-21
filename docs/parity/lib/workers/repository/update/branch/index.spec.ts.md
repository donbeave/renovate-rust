# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/index.spec.ts
**Total tests:** 101 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/branch/index › processBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips branch if not scheduled and branch does not exist | 157 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch creation if minimumGroupSize is not met | 167 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if not scheduled and not updating out of schedule | 180 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch for fresh release with minimumReleaseAge | 198 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if minimumReleaseAge not met | 223 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |

### `workers/repository/update/branch/index › processBranch › if release is missing releaseTimestamp with minimumReleaseAge set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips branch if minimumReleaseAgeBehaviour=timestamp-required | 241 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does not skip branch if minimumReleaseAgeBehaviour=timestamp-optional | 260 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does not skip branch if minimumReleaseAgeBehaviour=timestamp-required and minimumReleaseAge=0 days | 275 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if minimumConfidence not met | 291 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| processes branch if minimumConfidence is met | 310 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| processes branch if not scheduled but updating out of schedule | 329 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if closed major PR found | 343 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if closed digest PR found | 358 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| allows branch but disables automerge if merged PR found | 373 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if closed minor PR found | 388 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| allows branch even if merged PR found | 402 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| throws error if closed PR found | 418 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does not skip branch if edited PR found with rebaseLabel | 432 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if edited PR found | 451 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if tagretBranch of update PR is changed by user | 478 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if edited PR found without commenting | 510 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if target branch changed | 534 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if branch edited and no PR found | 570 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| continues branch if branch edited and but PR found | 581 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch if branch edited and and PR found with sha mismatch | 595 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch creation limit exceeded | 607 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch does not exist and in silent mode | 624 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch needs dependencyDashboardApproval | 642 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if pr creation limit exceeded and branch exists | 660 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if commits per run limit exceeded | 683 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does not return if commits per run limit exceeded but rebase requested | 707 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if commits hourly limit exceeded | 727 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does not return if commits hourly limit exceeded but rebase requested | 746 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if no work | 768 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if pending checks | 785 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if pending checks - but branch exists | 801 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| automerges when there is no pr and, pr-creation is off-schedule | 826 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch automerged | 847 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch automerged and no checks | 865 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch automerged (dry-run) | 886 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch exists and prCreation set to approval | 906 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch exists but pending | 931 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch automerge is pending | 957 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if PR creation failed | 983 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| handles unknown PrBlockedBy | 1009 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| retries setting branch status checks after PR creation | 1035 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does not retry setting branch status checks when PR is not created | 1067 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns if branch exists but updated | 1099 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| updates branch when no fingerprint match | 1130 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| updates branch when forceRebase=true | 1162 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and comments notice | 1194 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| fetches changelogs for the "branch" stage | 1228 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and tries automerge | 1247 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR when impossible to automerge | 1271 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR when impossible to automerge with mismatch keepUpdatedLabel | 1299 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips when automerge is off schedule | 1328 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR when impossible to automerge because off schedule | 1364 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and adds lock file error comment with default message if no releaseTimestamp | 1392 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and adds lock file error comment with user configured message if no releaseTimestamp | 1422 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and adds lock file error comment with templated user configured message if no releaseTimestamp | 1458 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and adds lock file error comment if old releaseTimestamp | 1495 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and adds lock file error comment if new releaseTimestamp and branch exists | 1520 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| throws error if lock file errors and new releaseTimestamp | 1545 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| ensures PR and adds lock file error comment recreate closed | 1568 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| swallows branch errors | 1593 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| throws and swallows branch errors | 1606 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| rebases branch onto new basebranch if baseBranch changed by user | 1627 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| rebases branch onto new basebranch if no fingerprint found | 1660 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| rebases branch onto new basebranch if no fingerprint found - 2 | 1700 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| swallows pr errors | 1735 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| closed pr (dry run) | 1761 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| branch pr no rebase (dry run) | 1776 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| branch pr no schedule lockfile (dry run) | 1797 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| branch pr no schedule (dry run) | 1842 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| branch pr no schedule | 1890 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch update if stopUpdatingLabel presents | 1932 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| skips branch update if same updates | 1968 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| updates branch if stopUpdatingLabel presents and PR rebase/retry box checked | 1996 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| updates branch if stopUpdatingLabel presents and dependency dashboard box checked | 2038 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| executes post-upgrade tasks if trust is high | 2077 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| handles post-upgrade task exec errors | 2176 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| executes post-upgrade tasks with disabled post-upgrade command templating | 2260 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| executes post-upgrade tasks with multiple dependecy in one branch | 2354 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| executes post-upgrade tasks once when set to branch mode | 2521 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| executes post-upgrade tasks with propagated post-upgrade file path via env variable | 2650 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| should not propagate post-upgrade file path via env variable if the post-upgrade file creation failed | 2755 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns when rebaseWhen=never | 2862 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| continues when rebaseWhen=never and keepUpdatedLabel | 2878 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| returns when rebaseWhen=never and keepUpdatedLabel does not match | 2905 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| continues when rebaseWhen=never but checked | 2932 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| continues when checked by checkedBranches | 2952 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does nothing when branchPrefixOld/branch and its pr exists | 2973 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| does nothing when branchPrefixOld/branch and its pr exists but updates not necessary | 3012 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| Dependency Dashboard All Pending approval | 3051 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| Dependency Dashboard open all rate-limited | 3088 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| Dependency Dashboard open all awaiting schedule | 3125 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| continues branch, skips automerge if there are artifact errors | 3162 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| continues to update PR, if branch got updated, even when prCreation!==immediate | 3189 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| checks out baseBranch after committing files | 3222 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| should not reattempt platform automerge without commitSha | 3244 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |
| should not reattempt platform automerge in dry run | 3268 | not-applicable | — | — | tests full branch update orchestration (git ops + platform APIs); out of scope for Rust extraction layer |

---

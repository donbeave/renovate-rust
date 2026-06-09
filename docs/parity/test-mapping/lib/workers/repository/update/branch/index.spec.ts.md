# `lib/workers/repository/update/branch/index.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**2/101 in-scope tests ported** (99 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 157 | skips branch if not scheduled and branch does not exist | ported | [`crates/renovate-core/src/workers/repository/update/branch/index.rs:345`](../../../../../../../../crates/renovate-core/src/workers/repository/update/branch/index.rs#L345) |
| 167 | skips branch creation if minimumgroupsize is not met | pending | — |
| 180 | skips branch if not scheduled and not updating out of schedule | pending | — |
| 198 | skips branch for fresh release with minimumreleaseage | pending | — |
| 223 | skips branch if minimumreleaseage not met | pending | — |
| 241 | skips branch if minimumreleaseagebehaviour=timestamp-required | pending | — |
| 260 | does not skip branch if minimumreleaseagebehaviour=timestamp-optional | pending | — |
| 275 | does not skip branch if minimumreleaseagebehaviour=timestamp-required and minimumreleaseage=0 days | pending | — |
| 291 | skips branch if minimumconfidence not met | pending | — |
| 310 | processes branch if minimumconfidence is met | pending | — |
| 329 | processes branch if not scheduled but updating out of schedule | pending | — |
| 343 | skips branch if closed major pr found | pending | — |
| 358 | skips branch if closed digest pr found | pending | — |
| 373 | allows branch but disables automerge if merged pr found | pending | — |
| 388 | skips branch if closed minor pr found | pending | — |
| 402 | allows branch even if merged pr found | pending | — |
| 418 | throws error if closed pr found | pending | — |
| 432 | does not skip branch if edited pr found with rebaselabel | pending | — |
| 451 | skips branch if edited pr found | ported | [`crates/renovate-core/src/workers/repository/update/branch/handle_existing.rs:187`](../../../../../../../../crates/renovate-core/src/workers/repository/update/branch/handle_existing.rs#L187) |
| 478 | skips branch if tagretbranch of update pr is changed by user | pending | — |
| 510 | skips branch if edited pr found without commenting | pending | — |
| 534 | skips branch if target branch changed | pending | — |
| 570 | skips branch if branch edited and no pr found | pending | — |
| 581 | continues branch if branch edited and but pr found | pending | — |
| 595 | skips branch if branch edited and and pr found with sha mismatch | pending | — |
| 607 | returns if branch creation limit exceeded | pending | — |
| 624 | returns if branch does not exist and in silent mode | pending | — |
| 642 | returns if branch needs dependencydashboardapproval | pending | — |
| 660 | returns if pr creation limit exceeded and branch exists | pending | — |
| 683 | returns if commits per run limit exceeded | pending | — |
| 707 | does not return if commits per run limit exceeded but rebase requested | pending | — |
| 727 | returns if commits hourly limit exceeded | pending | — |
| 746 | does not return if commits hourly limit exceeded but rebase requested | pending | — |
| 768 | returns if no work | pending | — |
| 785 | returns if pending checks | pending | — |
| 801 | returns if pending checks - but branch exists | pending | — |
| 826 | automerges when there is no pr and, pr-creation is off-schedule | pending | — |
| 847 | returns if branch automerged | pending | — |
| 865 | returns if branch automerged and no checks | pending | — |
| 886 | returns if branch automerged (dry-run) | pending | — |
| 906 | returns if branch exists and prcreation set to approval | pending | — |
| 931 | returns if branch exists but pending | pending | — |
| 957 | returns if branch automerge is pending | pending | — |
| 983 | returns if pr creation failed | pending | — |
| 1009 | handles unknown prblockedby | pending | — |
| 1035 | retries setting branch status checks after pr creation | pending | — |
| 1067 | does not retry setting branch status checks when pr is not created | pending | — |
| 1099 | returns if branch exists but updated | pending | — |
| 1130 | updates branch when no fingerprint match | pending | — |
| 1162 | updates branch when forcerebase=true | pending | — |
| 1194 | ensures pr and comments notice | pending | — |
| 1228 | fetches changelogs for the "branch" stage | pending | — |
| 1247 | ensures pr and tries automerge | pending | — |
| 1271 | ensures pr when impossible to automerge | pending | — |
| 1299 | ensures pr when impossible to automerge with mismatch keepupdatedlabel | pending | — |
| 1328 | skips when automerge is off schedule | pending | — |
| 1364 | ensures pr when impossible to automerge because off schedule | pending | — |
| 1392 | ensures pr and adds lock file error comment with default message if no releasetimestamp | pending | — |
| 1422 | ensures pr and adds lock file error comment with user configured message if no releasetimestamp | pending | — |
| 1458 | ensures pr and adds lock file error comment with templated user configured message if no releasetimestamp | pending | — |
| 1495 | ensures pr and adds lock file error comment if old releasetimestamp | pending | — |
| 1520 | ensures pr and adds lock file error comment if new releasetimestamp and branch exists | pending | — |
| 1545 | throws error if lock file errors and new releasetimestamp | pending | — |
| 1568 | ensures pr and adds lock file error comment recreate closed | pending | — |
| 1593 | swallows branch errors | pending | — |
| 1606 | throws and swallows branch errors | pending | — |
| 1627 | rebases branch onto new basebranch if basebranch changed by user | pending | — |
| 1660 | rebases branch onto new basebranch if no fingerprint found | pending | — |
| 1700 | rebases branch onto new basebranch if no fingerprint found - 2 | pending | — |
| 1735 | swallows pr errors | pending | — |
| 1761 | closed pr (dry run) | pending | — |
| 1776 | branch pr no rebase (dry run) | pending | — |
| 1797 | branch pr no schedule lockfile (dry run) | pending | — |
| 1842 | branch pr no schedule (dry run) | pending | — |
| 1890 | branch pr no schedule | pending | — |
| 1932 | skips branch update if stopupdatinglabel presents | pending | — |
| 1968 | skips branch update if same updates | pending | — |
| 1996 | updates branch if stopupdatinglabel presents and pr rebase/retry box checked | pending | — |
| 2038 | updates branch if stopupdatinglabel presents and dependency dashboard box checked | pending | — |
| 2077 | executes post-upgrade tasks if trust is high | pending | — |
| 2176 | handles post-upgrade task exec errors | pending | — |
| 2260 | executes post-upgrade tasks with disabled post-upgrade command templating | pending | — |
| 2354 | executes post-upgrade tasks with multiple dependecy in one branch | pending | — |
| 2521 | executes post-upgrade tasks once when set to branch mode | pending | — |
| 2650 | executes post-upgrade tasks with propagated post-upgrade file path via env variable | pending | — |
| 2755 | should not propagate post-upgrade file path via env variable if the post-upgrade file creation failed | pending | — |
| 2862 | returns when rebasewhen=never | pending | — |
| 2878 | continues when rebasewhen=never and keepupdatedlabel | pending | — |
| 2905 | returns when rebasewhen=never and keepupdatedlabel does not match | pending | — |
| 2932 | continues when rebasewhen=never but checked | pending | — |
| 2952 | continues when checked by checkedbranches | pending | — |
| 2973 | does nothing when branchprefixold/branch and its pr exists | pending | — |
| 3012 | does nothing when branchprefixold/branch and its pr exists but updates not necessary | pending | — |
| 3051 | dependency dashboard all pending approval | pending | — |
| 3088 | dependency dashboard open all rate-limited | pending | — |
| 3125 | dependency dashboard open all awaiting schedule | pending | — |
| 3162 | continues branch, skips automerge if there are artifact errors | pending | — |
| 3189 | continues to update pr, if branch got updated, even when prcreation!==immediate | pending | — |
| 3222 | checks out basebranch after committing files | pending | — |
| 3244 | should not reattempt platform automerge without commitsha | pending | — |
| 3268 | should not reattempt platform automerge in dry run | pending | — |


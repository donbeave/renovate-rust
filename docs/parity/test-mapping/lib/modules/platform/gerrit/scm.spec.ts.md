# `lib/modules/platform/gerrit/scm.spec.ts`

[← `platform/gerrit`](../../../../_by-module/platform/gerrit.md) · [all modules](../../../../README.md)

**0/29 in-scope tests ported** (29 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 29 | no open change for with branchname found -> isbehind == true | pending | — |
| 46 | open change found for branchname, rebase action is available -> isbehind == true | pending | — |
| 65 | open change found for branch name, but rebase action is not available -> isbehind == false | pending | — |
| 84 | no open change for with branchname found -> not modified | pending | — |
| 101 | open change found for branchname, but not modified | pending | — |
| 116 | open change found for branchname, but modified from other user | pending | — |
| 133 | no open change with branch name found -> return true | pending | — |
| 149 | open change found for branch name/basebranch and its mergeable | pending | — |
| 164 | open change found for branch name/basebranch and its not mergeable | pending | — |
| 181 | no change found for branch name -> return result from git.branchexists | pending | — |
| 196 | open change found for branch name -> return true | pending | — |
| 207 | no change found for branch name -> return result from git.getbranchcommit | pending | — |
| 224 | open change found for branchname -> return true | pending | — |
| 234 | no change found for branch name -> return result from git.getbranchupdatedate | pending | — |
| 258 | open change found for branchname -> return datetime from gerrit change | pending | — |
| 280 | pushes to refs/for/<targetbranch> and returns true on success | pending | — |
| 297 | adds hashtag push options for each label | pending | — |
| 320 | clears pending change branch on success | pending | — |
| 333 | keeps pending change branch when push fails | pending | — |
| 348 | deletes local branch | pending | — |
| 355 | clears pending change branch | pending | — |
| 363 | no change exists | pending | — |
| 383 | uses local merge when there is a pending change branch | pending | — |
| 394 | change exists | pending | — |
| 424 | commitandpush() - empty commit | pending | — |
| 448 | commitandpush() - create first commit but does not push | pending | — |
| 482 | commitandpush() - existing change keeps original target branch | pending | — |
| 531 | commitandpush() - existing change without new changes | pending | — |
| 575 | commitandpush() - existing change with new changes - auto-approve | pending | — |


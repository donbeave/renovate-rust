# `lib/workers/repository/update/pr/index.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/53 in-scope tests ported** (53 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 78 | creates pr | pending | — |
| 98 | fetches changelogs for the "pr" stage | pending | — |
| 109 | aborts pr creation once limit is exceeded | pending | — |
| 122 | ignores pr limits on vulnerability alert | pending | — |
| 135 | creates rollback pr | pending | — |
| 146 | skips pr creation due to non-green branch check | pending | — |
| 158 | creates pr for green branch checks | pending | — |
| 169 | skips pr creation for unapproved dependencies | pending | — |
| 181 | skips pr creation before prnotpendinghours is hit | pending | — |
| 201 | skips pr creation due to stabilitystatus | pending | — |
| 221 | creates pr after prnotpendinghours is hit | pending | — |
| 240 | handles unknown error | pending | — |
| 250 | handles error for pr that already exists | pending | — |
| 268 | deletes branch on 502 error | pending | — |
| 285 | updates pr if labels have changed in config | pending | — |
| 339 | skips pr update if existing pr does not have labels in debugdata | pending | — |
| 366 | skips pr update if pr labels have been modified by user | pending | — |
| 418 | updates pr due to title change | pending | — |
| 435 | updates pr due to body change | pending | — |
| 455 | updates pr target branch if base branch changed in config | pending | — |
| 483 | ignores reviewable content | pending | — |
| 512 | dry-runs pr creation | pending | — |
| 530 | dry-runs pr update | pending | — |
| 545 | skips automerge failure comment | pending | — |
| 563 | handles branch automerge | pending | — |
| 579 | forces pr on dashboard check | pending | — |
| 596 | adds assignees for pr automerge with red status | pending | — |
| 616 | adds reviewers for pr automerge with red status and existing ignorable reviewers that can be ignored | pending | — |
| 637 | skips branch automerge and forces pr creation due to artifact errors | pending | — |
| 653 | skips branch automerge and forces pr creation due to prnotpendinghours exceeded | pending | — |
| 674 | automerges branch when prnotpendinghours are not exceeded | pending | — |
| 697 | comments on automerge failure | pending | — |
| 720 | handles ensurecomment error | pending | — |
| 737 | logs unknown error | pending | — |
| 761 | re-throws externalhosterror | pending | — |
| 782 | _(it.each / template — verify manually)_ | ? | — |
| 854 | processes changelogs | pending | — |
| 880 | handles missing github token | pending | — |
| 902 | removes duplicate changelogs | pending | — |
| 928 | remove duplicates release notes | pending | — |
| 958 | stricter de-deuplication of changelogs | pending | — |
| 1026 | does not warn the user | pending | — |
| 1056 | warns the user | pending | — |
| 1104 | does not warn the user | pending | — |
| 1128 | adds pr-cache when not present | pending | — |
| 1144 | does not update lastedited pr-cache when pr fingerprint is same but pr was edited within 24hrs | pending | — |
| 1171 | updates pr-cache when pr fingerprint is different | pending | — |
| 1190 | skips fetching changelogs when cache is valid and pr was lastedited before 24hrs | pending | — |
| 1215 | updates pr when rebase requested by user regardless of pr-cache state | pending | — |
| 1259 | logs when cache is enabled but pr-cache is absent | pending | — |
| 1268 | does not log when cache is disabled and pr-cache is absent | pending | — |
| 1278 | skips cache early return when autoapprove is set | pending | — |
| 1300 | updates pr when autoapprove is set even if pr does not need updating | pending | — |


# `lib/modules/platform/gerrit/client.spec.ts`

[← `platform/gerrit`](../../../../_by-module/platform/gerrit.md) · [all modules](../../../../README.md)

**0/43 ported** (43 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 24 | returns version | pending | — |
| 39 | returns repos | pending | — |
| 56 | inactive | pending | — |
| 74 | active | pending | — |
| 96 | info | pending | — |
| 113 | _(it.each / template — verify manually)_ | ? | — |
| 191 | sets query.n as 1 if a single change is requested | pending | — |
| 206 | sets query.n as 50 if pagelimit is not provided | pending | — |
| 219 | sets query.n with pagelimit if provided | pending | — |
| 233 | sets query.s with startoffset if provided | pending | — |
| 247 | sets query.s as 0 if startoffset is not provided | pending | — |
| 260 | handles pagination automatically | pending | — |
| 305 | handles pagination with startoffset | pending | — |
| 339 | allows disabling automatic pagination | pending | — |
| 361 | sets query.o when requestdetails is provided | pending | — |
| 381 | get | pending | — |
| 394 | get | pending | — |
| 410 | abandon | pending | — |
| 419 | abandon with message | pending | — |
| 434 | submit | pending | — |
| 445 | move change to different branch | pending | — |
| 460 | returns null when no changes found | pending | — |
| 474 | returns single change when only one found | pending | — |
| 492 | returns first change when multiple found without targetbranch | pending | — |
| 514 | returns matching change when targetbranch specified and match found | pending | — |
| 537 | returns first change when targetbranch specified but no match found | pending | — |
| 562 | no messages | pending | — |
| 570 | with messages | pending | — |
| 590 | add with tag | pending | — |
| 602 | add without tag | pending | — |
| 613 | add too big message | pending | — |
| 633 | msg not found | pending | — |
| 643 | msg found | pending | — |
| 662 | msg not found | pending | — |
| 685 | msg already exists | pending | — |
| 704 | setlabel | pending | — |
| 719 | add hashtags | pending | — |
| 731 | remove hashtags | pending | — |
| 743 | add and remove hashtags in single call | pending | — |
| 759 | does nothing when no hashtags provided | pending | — |
| 770 | add | pending | — |
| 783 | add | pending | — |
| 795 | getfile() - repo and branch | pending | — |


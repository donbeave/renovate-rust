# `lib/workers/repository/update/branch/auto-replace.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**1/71 in-scope tests ported** (70 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 36 | rebases if the deps list has changed | pending | — |
| 47 | rebases if the deps to update has changed | pending | — |
| 56 | uses depname or packagename | pending | — |
| 78 | updates version only | pending | — |
| 93 | handles a double attempt | pending | — |
| 107 | handles already updated | pending | — |
| 128 | handles no work | pending | — |
| 144 | returns existing content if replacestring mismatch | pending | — |
| 163 | updates version and integrity | pending | — |
| 182 | updates with autoreplacenewstring | ported | [`crates/renovate-core/src/workers/repository/update/branch/auto_replace.rs:315`](../../../../../../../../crates/renovate-core/src/workers/repository/update/branch/auto_replace.rs#L315) |
| 204 | succeeds when using autoreplacestringtemplate to update depname when using regex | pending | — |
| 244 | fails with oldversion in depname | pending | — |
| 266 | updates digest when only digest changes and no replacestring is set | pending | — |
| 284 | updates with docker replacement | pending | — |
| 298 | handles already replaced | pending | — |
| 309 | handles replacement with depname===newname when replacestring exists | pending | — |
| 333 | updates with terraform replacement | pending | — |
| 350 | updates with ansible replacement | pending | — |
| 376 | updates with ansible-galaxy roles replacement | pending | — |
| 398 | updates with azure-pipeline image replacement | pending | — |
| 422 | updates with batect image replacement | pending | — |
| 445 | updates with bitbucket-pipelines image replacement | pending | — |
| 464 | updates with buildkite plugin replacement | pending | — |
| 487 | updates with bundler gem replacement | pending | — |
| 509 | updates with cake #addin replacement | pending | — |
| 528 | updates with cargo dependency replacement | pending | — |
| 549 | updates with cloudbuild replacement | pending | — |
| 572 | updates with podfile pod replacement | pending | — |
| 590 | updates with composer require replacement | pending | — |
| 614 | updates with edn deps replacement | pending | — |
| 637 | updates with docker-compose image replacement | pending | — |
| 660 | updates with dockerfile image replacement | pending | — |
| 679 | updates with dockerfile image replacement with digest | pending | — |
| 701 | updates with droneci image replacement | pending | — |
| 724 | updates with gitlabci image replacement | pending | — |
| 743 | updates with helm value image/repository replacement | pending | — |
| 767 | updates with helm value image/repository replacement with digest | pending | — |
| 794 | updates with helm value image/repository wrong version | pending | — |
| 816 | updates with helm value image/repository prefix replacement | pending | — |
| 840 | updates with helm value image/repository version prefix replacement | pending | — |
| 864 | updates with jenkins plugin replacement | pending | — |
| 882 | updates with meteor npm.depends replacement | pending | — |
| 908 | checks for replacewithoutreplacestring double update | pending | — |
| 942 | updates with mix deps replacement | pending | — |
| 983 | updates with nuget tools replacement | pending | — |
| 1011 | updates with pre-commit repo replacement | pending | — |
| 1033 | updates with terraform image replacement | pending | — |
| 1056 | updates with terraform module replacement | pending | — |
| 1079 | updates with setup-cfg replacement | pending | — |
| 1100 | updates with nvm version replacement | pending | — |
| 1118 | updates with multiple same name replacement without replacestring | pending | — |
| 1142 | updates with multiple same name replacement without replacestring 2 | pending | — |
| 1166 | updates with multiple same version replacement without replacestring | pending | — |
| 1190 | updates with multiple same digest replacement without replacestring | pending | — |
| 1216 | docker: updates with pindigest enabled but no currentdigest value | pending | — |
| 1240 | docker: updates with pindigest enabled and a currentdigest value | pending | — |
| 1264 | autoreplaceglobalmatch: throws error when globally replacing recurring values across version and digests | pending | — |
| 1284 | autoreplaceglobalmatch: updates when replacing first match only of recurring values across version and digests | pending | — |
| 1309 | regex: updates with pindigest enabled but no currentdigest value | pending | — |
| 1333 | regex: updates with pindigest enabled and a currentdigest value | pending | — |
| 1361 | jsonata: update currentvalue | pending | — |
| 1385 | jsonata: update currentdigest | pending | — |
| 1408 | jsonata: update currentvalue and currentdigest | pending | — |
| 1433 | jsonata: update currentdigest with currentvalue captured | pending | — |
| 1457 | github-actions: updates with newvalue only | pending | — |
| 1494 | github-actions: updates with newvalue and newdigest | pending | — |
| 1531 | github-actions: updates with pindigest enabled but no currentdigest value | pending | — |
| 1570 | github-actions: updates with pindigest enabled and a currentdigest value | pending | — |
| 1608 | github-actions: failes to update currentdigestshort | pending | — |
| 1646 | docker: replacement with same digest should not corrupt digest via currentdigestshort | pending | — |
| 1677 | updates only digest | pending | — |


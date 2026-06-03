# `lib/modules/manager/helmv3/artifacts.spec.ts`

[← `manager/helmv3`](../../../../_by-module/manager/helmv3.md) · [all modules](../../../../README.md)

**0/24 in-scope tests ported** (24 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 72 | returns null if no chart.lock found | pending | — |
| 84 | returns null if updateddeps is empty | pending | — |
| 95 | returns null if unchanged | pending | — |
| 116 | returns null if only "generated" is changed | pending | — |
| 155 | returns updated chart.lock | pending | — |
| 185 | returns updated chart.lock for lockfile maintenance | pending | — |
| 214 | returns updated chart.lock with docker | pending | — |
| 252 | catches errors | pending | — |
| 279 | add sub chart artifacts to file list if chart.lock exists | pending | — |
| 339 | add sub chart artifacts to file list if chart.lock is missing | pending | — |
| 414 | add sub chart artifacts without old archives | pending | — |
| 482 | add sub chart artifacts and ignore files outside of the chart folder | pending | — |
| 557 | skip artifacts which are not lock files or in the chart folder | pending | — |
| 617 | sets repositories from registryaliases ignoring not well formed uri | pending | — |
| 654 | sets repositories from registryaliases with docker | pending | — |
| 699 | log into private registries and repositories already defined in registryaliases | pending | — |
| 749 | log into private registries and repositories not defined in registryaliases | pending | — |
| 795 | supports ecr authentication | pending | — |
| 861 | does not use ecr authentication when the host rule's username is aws | pending | — |
| 918 | continues without auth if the ecr token is invalid | pending | — |
| 979 | continues without auth if ecr authentication fails | pending | — |
| 1038 | alias name is picked, when repository is as alias and dependency defined | pending | — |
| 1093 | do not add registryaliases to repository list | pending | — |
| 1142 | prevents injections | pending | — |


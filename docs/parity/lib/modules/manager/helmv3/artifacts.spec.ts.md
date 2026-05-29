# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmv3/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmv3/artifacts.spec.ts
**Total tests:** 24 | **Ported:** 0 | **Actionable:** 24 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no Chart.lock found | 71 | pending | — | — | — |
| returns null if updatedDeps is empty | 83 | pending | — | — | — |
| returns null if unchanged | 94 | pending | — | — | — |
| returns null if only "generated" is changed | 115 | pending | — | — | — |
| returns updated Chart.lock | 154 | pending | — | — | — |
| returns updated Chart.lock for lockfile maintenance | 184 | pending | — | — | — |
| returns updated Chart.lock with docker | 213 | pending | — | — | — |
| catches errors | 251 | pending | — | — | — |
| add sub chart artifacts to file list if Chart.lock exists | 278 | pending | — | — | — |
| add sub chart artifacts to file list if Chart.lock is missing | 338 | pending | — | — | — |
| add sub chart artifacts without old archives | 413 | pending | — | — | — |
| add sub chart artifacts and ignore files outside of the chart folder | 481 | pending | — | — | — |
| skip artifacts which are not lock files or in the chart folder | 556 | pending | — | — | — |
| sets repositories from registryAliases ignoring not well formed URI | 616 | pending | — | — | — |
| sets repositories from registryAliases with docker | 653 | pending | — | — | — |
| log into private registries and repositories already defined in registryAliases | 698 | pending | — | — | — |
| log into private registries and repositories NOT defined in registryAliases | 748 | pending | — | — | — |
| supports ECR authentication | 794 | pending | — | — | — |
| does not use ECR authentication when the host rule's username is AWS | 860 | pending | — | — | — |
| continues without auth if the ECR token is invalid | 917 | pending | — | — | — |
| continues without auth if ECR authentication fails | 978 | pending | — | — | — |
| alias name is picked, when repository is as alias and dependency defined | 1037 | pending | — | — | — |
| do not add registryAliases to repository list | 1092 | pending | — | — | — |
| prevents injections | 1141 | pending | — | — | — |

---


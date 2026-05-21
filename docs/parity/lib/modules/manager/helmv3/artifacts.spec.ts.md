# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmv3/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmv3/artifacts.spec.ts
**Total tests:** 24 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no Chart.lock found | 71 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if updatedDeps is empty | 83 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if unchanged | 94 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if only "generated" is changed | 115 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Chart.lock | 154 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Chart.lock for lockfile maintenance | 184 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated Chart.lock with docker | 213 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors | 251 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| add sub chart artifacts to file list if Chart.lock exists | 278 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| add sub chart artifacts to file list if Chart.lock is missing | 338 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| add sub chart artifacts without old archives | 413 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| add sub chart artifacts and ignore files outside of the chart folder | 481 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skip artifacts which are not lock files or in the chart folder | 556 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| sets repositories from registryAliases ignoring not well formed URI | 616 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| sets repositories from registryAliases with docker | 653 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| log into private registries and repositories already defined in registryAliases | 698 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| log into private registries and repositories NOT defined in registryAliases | 748 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports ECR authentication | 794 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not use ECR authentication when the host rule's username is AWS | 860 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| continues without auth if the ECR token is invalid | 917 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| continues without auth if ECR authentication fails | 978 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| alias name is picked, when repository is as alias and dependency defined | 1037 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| do not add registryAliases to repository list | 1092 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| prevents injections | 1141 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


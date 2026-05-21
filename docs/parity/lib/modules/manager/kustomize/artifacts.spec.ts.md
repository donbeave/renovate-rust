# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/kustomize/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kustomize/artifacts.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if newPackageFileContent is not parseable | 46 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no HelmChart dependencies found | 69 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no dependency name is found | 90 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no registryUrl is found | 111 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no packageName is found | 132 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if neither currentVersion or newVersion is found | 153 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if newVersion is not found and currentVersion is already inflated | 174 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if old version is not inflated and kustomizeInflateHelmCharts is not enabled | 197 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if newVersion and currentVersion is the same | 231 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| inflates new version if old version is inflated and kustomizeInflateHelmCharts is not enabled | 262 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| inflates new version if old version is not inflated but kustomizeInflateHelmCharts is enabled | 323 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| inflates current version if no new version and kustomizeInflateHelmCharts is enabled | 367 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles OCI repositories | 411 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| installs binaries on install mode | 455 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| installs binaries on docker mode | 505 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not inflate current version if kustomizeInflateHelmCharts is not enabled | 574 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors | 610 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| throws on TEMPORARY_ERROR | 648 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| prevents injections | 680 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


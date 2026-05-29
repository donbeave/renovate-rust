# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/kustomize/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kustomize/artifacts.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if newPackageFileContent is not parseable | 46 | pending | — | — | —|
| returns null if no HelmChart dependencies found | 69 | pending | — | — | —|
| returns null if no dependency name is found | 90 | pending | — | — | —|
| returns null if no registryUrl is found | 111 | pending | — | — | —|
| returns null if no packageName is found | 132 | pending | — | — | —|
| returns null if neither currentVersion or newVersion is found | 153 | pending | — | — | —|
| returns null if newVersion is not found and currentVersion is already inflated | 174 | pending | — | — | —|
| returns null if old version is not inflated and kustomizeInflateHelmCharts is not enabled | 197 | pending | — | — | —|
| returns null if newVersion and currentVersion is the same | 231 | pending | — | — | —|
| inflates new version if old version is inflated and kustomizeInflateHelmCharts is not enabled | 262 | pending | — | — | —|
| inflates new version if old version is not inflated but kustomizeInflateHelmCharts is enabled | 323 | pending | — | — | —|
| inflates current version if no new version and kustomizeInflateHelmCharts is enabled | 367 | pending | — | — | —|
| handles OCI repositories | 411 | pending | — | — | —|
| installs binaries on install mode | 455 | pending | — | — | —|
| installs binaries on docker mode | 505 | pending | — | — | —|
| does not inflate current version if kustomizeInflateHelmCharts is not enabled | 574 | pending | — | — | —|
| catches errors | 610 | pending | — | — | —|
| throws on TEMPORARY_ERROR | 648 | pending | — | — | —|
| prevents injections | 680 | pending | — | — | —|

---


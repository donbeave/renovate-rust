# `lib/modules/manager/kustomize/artifacts.spec.ts`

[← `manager/kustomize`](../../../../_by-module/manager/kustomize.md) · [all modules](../../../../README.md)

**0/19 ported** (19 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 47 | returns null if newpackagefilecontent is not parseable | pending | — |
| 70 | returns null if no helmchart dependencies found | pending | — |
| 91 | returns null if no dependency name is found | pending | — |
| 112 | returns null if no registryurl is found | pending | — |
| 133 | returns null if no packagename is found | pending | — |
| 154 | returns null if neither currentversion or newversion is found | pending | — |
| 175 | returns null if newversion is not found and currentversion is already inflated | pending | — |
| 198 | returns null if old version is not inflated and kustomizeinflatehelmcharts is not enabled | pending | — |
| 232 | returns null if newversion and currentversion is the same | pending | — |
| 263 | inflates new version if old version is inflated and kustomizeinflatehelmcharts is not enabled | pending | — |
| 324 | inflates new version if old version is not inflated but kustomizeinflatehelmcharts is enabled | pending | — |
| 368 | inflates current version if no new version and kustomizeinflatehelmcharts is enabled | pending | — |
| 412 | handles oci repositories | pending | — |
| 456 | installs binaries on install mode | pending | — |
| 506 | installs binaries on docker mode | pending | — |
| 575 | does not inflate current version if kustomizeinflatehelmcharts is not enabled | pending | — |
| 611 | catches errors | pending | — |
| 649 | throws on temporary_error | pending | — |
| 681 | prevents injections | pending | — |


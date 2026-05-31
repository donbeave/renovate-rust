# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/kustomize/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kustomize/artifacts.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if newPackageFileContent is not parseable | 46 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no HelmChart dependencies found | 69 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no dependency name is found | 90 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no registryUrl is found | 111 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no packageName is found | 132 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if neither currentVersion or newVersion is found | 153 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if newVersion is not found and currentVersion is already inflated | 174 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if old version is not inflated and kustomizeInflateHelmCharts is not enabled | 197 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if newVersion and currentVersion is the same | 231 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| inflates new version if old version is inflated and kustomizeInflateHelmCharts is not enabled | 262 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| inflates new version if old version is not inflated but kustomizeInflateHelmCharts is enabled | 323 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| inflates current version if no new version and kustomizeInflateHelmCharts is enabled | 367 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles OCI repositories | 411 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| installs binaries on install mode | 455 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| installs binaries on docker mode | 505 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| does not inflate current version if kustomizeInflateHelmCharts is not enabled | 574 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 610 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| throws on TEMPORARY_ERROR | 648 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| prevents injections | 680 | not-applicable | Mock framework internals — tests kustomize artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---


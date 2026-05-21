# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmfile/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmfile/artifacts.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no helmfile.lock found | 83 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if updatedDeps is empty | 95 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if unchanged | 106 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated helmfile.lock | 128 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated helmfile.lock if repositories were defined in ../helmfile-defaults.yaml. | 159 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| log into private OCI registries, returns updated helmfile.lock | 219 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| docker run --rm --name=renovate_sidecar --label=renovate_child | 310 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| not found | 391 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates lockfile with multidoc YAML | 421 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/kustomize/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kustomize/common.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `generateHelmEnvs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates envs for specific helm version not requiring HELM_EXPERIMENTAL_OCI | 19 | ported | `util.rs` | `test_helm_envs_no_experimental_oci_specific_version` | — |
| generates envs for helm version range not requiring HELM_EXPERIMENTAL_OCI | 34 | ported | `util.rs` | `test_helm_envs_no_experimental_oci_range` | — |
| generates envs for specific helm version requiring HELM_EXPERIMENTAL_OCI | 49 | ported | `util.rs` | `test_helm_envs_with_experimental_oci_specific` | — |
| generates envs for helm range version requiring HELM_EXPERIMENTAL_OCI | 66 | ported | `util.rs` | `test_helm_envs_with_experimental_oci_range` | — |

---


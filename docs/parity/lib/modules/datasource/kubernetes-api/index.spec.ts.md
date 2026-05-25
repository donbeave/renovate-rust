# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/kubernetes-api/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/kubernetes-api/index.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** done

### `modules/datasource/kubernetes-api/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for an unknown Kubernetes API type | 8 | ported | `crates/renovate-core/src/datasources/kubernetes_api.rs` | `unknown_type_returns_none` | Unknown type names return None |
| returns for a known Kubernetes API type | 13 | ported | `crates/renovate-core/src/datasources/kubernetes_api.rs` | `known_type_returns_versions` | CSIStorageCapacity → storage.k8s.io/v1beta1 and storage.k8s.io/v1 |
| is case sensitive | 27 | ported | `crates/renovate-core/src/datasources/kubernetes_api.rs` | `lookup_is_case_sensitive` | csistoragecapacity → None, CSIStorageCapacity → Some |

---


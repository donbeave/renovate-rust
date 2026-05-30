# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmv3/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmv3/utils.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 0 | **Status:** done

### `.resolveAlias()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return alias with "alias:" | 6 | ported | `helm.rs` | `helm_resolve_alias_with_alias_prefix` | — |
| return alias with "@" | 14 | ported | `helm.rs` | `helm_resolve_alias_with_at_prefix` | — |
| return null if alias repo is not defined | 22 | ported | `helm.rs` | `helm_resolve_alias_undefined_returns_none` | — |
| return resolved repository on OCI registries | 29 | ported | `helm.rs` | `helm_resolve_alias_oci_registry` | — |
| return repository parameter if it is not an alias | 36 | ported | `helm.rs` | `helm_resolve_alias_non_alias_passthrough` | — |
| return repository parameter if repository is null | 44 | ported | `helm.rs` | `helm_resolve_alias_null_returns_none` | — |
| return repository parameter if repository is undefined | 52 | ported | `helm.rs` | `helm_resolve_alias_undefined_input_returns_none` | — |

### `.isAlias()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return false if repository is null | 62 | ported | `helm.rs` | `helm_is_alias_null_returns_false` | — |
| return false if repository is undefined | 68 | ported | `helm.rs` | `helm_is_alias_undefined_returns_false` | — |

### `.isOCIRegistry()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return false if repository is null | 76 | ported | `helm.rs` | `helm_is_oci_registry_null_returns_false` | — |
| return false if repository is undefined | 81 | ported | `helm.rs` | `helm_is_oci_registry_undefined_returns_false` | — |

---


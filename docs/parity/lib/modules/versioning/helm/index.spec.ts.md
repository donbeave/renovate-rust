# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/helm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/helm/index.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `modules/versioning/helm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $isValid | 4 | ported | `helm.rs` | `is_valid_matches_renovate_helm_spec` | — |
| isSingleVersion("$version") === $isSingle | 22 | ported | `helm.rs` | `is_single_version_matches_renovate_helm_spec` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 35 | ported | `helm.rs` | `get_new_value_matches_renovate_helm_spec` | — |

---


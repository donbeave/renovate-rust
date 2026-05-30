# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/terragrunt/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terragrunt/util.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `getTerragruntDependencyType()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns terraform | 5 | ported | `terragrunt.rs` | `get_dependency_type_returns_terraform` | — |
| returns unknown | 9 | ported | `terragrunt.rs` | `get_dependency_type_returns_unknown` | — |
| returns unknown on empty string | 13 | ported | `terragrunt.rs` | `get_dependency_type_returns_unknown_for_empty` | — |
| returns unknown on string with random chars | 17 | ported | `terragrunt.rs` | `get_dependency_type_returns_unknown_for_random` | — |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/terraform-version/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform-version/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `terraform_version_plain` | — |
| skips non ranges | 18 | ported | `version_file.rs` | `terraform_version_passes_through_non_alias_literal` | — |

---


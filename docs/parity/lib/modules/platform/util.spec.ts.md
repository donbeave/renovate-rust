# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/util.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** done

### `modules/platform/util › repoFingerprint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ("$repoId", "$endpoint") === $fingerprint | 8 | ported | `platform/util.rs` | `platform_util_repo_fingerprint` | — |

### `modules/platform/util › getNewBranchName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add refs/heads | 21 | ported | `platform/util.rs` | `platform_util_get_new_branch_name_adds_prefix` | — |
| should be the same | 26 | ported | `platform/util.rs` | `platform_util_get_new_branch_name_keeps_prefix` | — |

---

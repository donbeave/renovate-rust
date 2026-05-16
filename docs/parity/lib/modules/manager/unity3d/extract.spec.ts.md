# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/unity3d/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/unity3d/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles no version | 5 | ported | `unity3d.rs` | `returns_none_for_empty` | — |
| handles $packageName | 14 | ported | `unity3d.rs` | `extracts_plain_version` (+ extracts_with_revision_version) | — |
| handles $type version | 39 | ported | `unity3d.rs` | `extracts_alpha_beta_and_stable_versions_with_revisions` | — |

---


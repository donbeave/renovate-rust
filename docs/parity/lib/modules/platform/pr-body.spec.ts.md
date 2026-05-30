# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/pr-body.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/pr-body.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `modules/platform/pr-body › getPrBodyStruct`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns hash for empty inputs | 6 | ported | `platform/pr_body.rs` | `pr_body_struct_empty_inputs` | — |
| checks if we reach warning | 29 | ported | `platform/pr_body.rs` | `pr_body_struct_invalid_debug` | — |
| hashes ignoring debug info | 39 | ported | `platform/pr_body.rs` | `pr_body_hash_ignores_debug` | — |
| hashes ignoring reviewable section | 45 | ported | `platform/pr_body.rs` | `pr_body_hash_ignores_reviewable` | — |
| hashes an undefined body | 51 | ported | `platform/pr_body.rs` | `pr_body_hash_undefined` | — |
| returns rebaseRequested=true flag | 58 | ported | `platform/pr_body.rs` | `pr_body_rebase_true` | — |
| returns rebaseRequested=false flag | 67 | ported | `platform/pr_body.rs` | `pr_body_rebase_false` | — |
| returns rebaseRequested=undefined flag | 76 | ported | `platform/pr_body.rs` | `pr_body_rebase_none` | — |
| returns raw config hash | 84 | ported | `platform/pr_body.rs` | `pr_body_raw_config_hash` | — |
| strips reviewable section | 95 | ported | `platform/pr_body.rs` | `pr_body_strips_reviewable` | — |

---

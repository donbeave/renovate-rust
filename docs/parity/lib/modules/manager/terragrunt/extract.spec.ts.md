# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/terragrunt/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terragrunt/extract.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `terragrunt.rs` | `no_terraform_block_returns_empty` | — |
| extracts terragrunt sources using tfr protocol | 10 | ported | `terragrunt.rs` | `extracts_tfr_protocol_sources` | — |
| extracts terragrunt sources | 51 | ported | `terragrunt.rs` | `extracts_github_ref_source` (+ extracts_git_prefix_github, multiple_terraform_blocks, local_path_skipped) | — |
| extracts terragrunt sources with depth specified after the branch | 269 | ported | `terragrunt.rs` | `extracts_sources_with_depth_after_ref` | — |
| extracts terragrunt sources with depth specified before the branch | 487 | ported | `terragrunt.rs` | `extracts_sources_with_depth_before_ref` | — |
| returns null if only local terragrunt deps | 698 | ported | `terragrunt.rs` | `local_only_deps_returns_empty` | — |
| returns empty deps if only local terragrunt includes | 707 | ported | `terragrunt.rs` | `include_block_only_returns_empty` | — |

---


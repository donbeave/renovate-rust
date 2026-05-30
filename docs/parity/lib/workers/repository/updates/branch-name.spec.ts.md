# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/updates/branch-name.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/updates/branch-name.spec.ts
**Total tests:** 27 | **Ported:** 27 | **Actionable:** 0 | **Status:** done

### `workers/repository/updates/branch-name › getBranchName()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| falls back to sharedVariableName if no groupName | 7 | ported | `branch.rs` | `branch_name_falls_back_to_shared_variable_name` | — |
| ignores grouping of replacement update | 19 | ported | `branch.rs` | `branch_name_ignores_grouping_for_replacement_update` | — |
| applies grouping for lockfile maintenance update | 36 | ported | `branch.rs` | `branch_name_applies_grouping_for_lockfile_maintenance` | — |
| uses default branch name for lockfile maintenance without groupName | 52 | ported | `branch.rs` | `branch_name_lockfile_maintenance_without_group_name` | — |
| separates lockFileMaintenance from non-lockFileMaintenance with same groupName | 63 | ported | `branch.rs` | `branch_name_separates_lockfile_from_non_lockfile_same_group` | — |
| uses groupName if no slug defined, ignores sharedVariableName | 89 | ported | `branch.rs` | `branch_name_uses_group_name_ignores_shared_variable_name` | — |
| compile groupName before slugging | 102 | ported | `branch.rs` | `branch_name_compiles_group_name_before_slugging` | — |
| uses groupSlug if defined | 115 | ported | `branch.rs` | `branch_name_uses_group_slug_if_defined` | — |
| separates major with groups | 129 | ported | `branch.rs` | `branch_name_separates_major_with_groups` | — |
| separates minor with groups | 146 | ported | `branch.rs` | `branch_name_separates_minor_with_groups` | — |
| separates minor when separateMultipleMinor=true | 163 | ported | `branch.rs` | `branch_name_separates_minor_separate_multiple_minor_true` | — |
| uses single major with groups | 183 | ported | `branch.rs` | `branch_name_uses_single_major_with_groups` | — |
| separates patch groups and uses update topic | 200 | ported | `branch.rs` | `branch_name_separates_patch_groups_uses_update_topic` | — |
| compiles multiple times | 218 | ported | `branch.rs` | `branch_name_compiles_multiple_times` | — |
| separates patches when separateMinorPatch=true | 229 | ported | `branch.rs` | `branch_name_separates_patches_when_separate_minor_patch_true` | — |
| does not separate patches when separateMinorPatch=false | 249 | ported | `branch.rs` | `branch_name_does_not_separate_patches_when_separate_minor_patch_false` | — |
| realistic defaults | 269 | ported | `branch.rs` | `branch_name_realistic_defaults` | — |
| realistic defaults with strict branch name enabled | 284 | ported | `branch.rs` | `branch_name_realistic_defaults_with_strict_enabled` | — |
| removes slashes from the non-suffix part | 300 | ported | `branch.rs` | `branch_name_strict_removes_slashes_from_non_suffix_part` | — |
| hashedBranchLength hashing | 316 | ported | `branch.rs` | `hashed_branch_length_hashing_matches_renovate` | — |
| hashedBranchLength hashing with group name | 332 | ported | `branch.rs` | `hashed_branch_length_hashing_with_group_name_matches_renovate` | — |
| hashedBranchLength too short | 350 | ported | `branch.rs` | `hashed_branch_length_too_short_matches_renovate_minimum` | — |
| hashedBranchLength no topic | 368 | ported | `branch.rs` | `hashed_branch_length_no_topic_matches_renovate_empty_hash` | — |
| hashedBranchLength separates minor when separateMultipleMinor=true | 386 | ported | `branch.rs` | `hashed_branch_length_separate_multiple_minor_matches_renovate` | — |
| enforces valid git branch name | 405 | ported | `branch.rs` | `branch_name_enforces_valid_git_branch_name` | — |
| strict branch name enabled group | 491 | ported | `branch.rs` | `branch_name_strict_enabled_group` | — |
| strict branch name disabled | 506 | ported | `branch.rs` | `branch_name_strict_disabled_group` | — |

---


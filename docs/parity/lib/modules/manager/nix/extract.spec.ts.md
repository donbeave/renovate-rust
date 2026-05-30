# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nix/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nix/extract.spec.ts
**Total tests:** 38 | **Ported:** 38 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no nixpkgs input exists | 10 | ported | `nix.rs` | `package_file_returns_none_when_no_nixpkgs_input_exists` | — |
| does not include nixpkgs input with no explicit ref | 25 | ported | `nix.rs` | `package_file_returns_none_for_nixpkgs_without_explicit_ref_when_lock_has_no_input` | — |
| includes nixpkgs input with only ref | 42 | ported | `nix.rs` | `package_file_returns_none_for_ref_only_flake_when_lock_has_no_input` | — |
| returns null when no inputs | 59 | ported | `nix.rs` | `package_file_returns_none_when_flake_nix_has_no_inputs` | — |
| returns null when inputs are missing locked | 71 | ported | `nix.rs` | `missing_locked_section_is_skipped_as_no_rev` | — |
| returns null when inputs are missing original | 95 | ported | `nix.rs` | `missing_original_section_is_skipped_as_no_rev` | — |
| returns null when original inputs are from local path | 121 | ported | `nix.rs` | `original_path_input_is_skipped_as_local_path` | — |
| returns null when locked inputs are indirect | 153 | ported | `nix.rs` | `locked_indirect_input_is_skipped` | — |
| returns null when locked inputs are from local path | 185 | ported | `nix.rs` | `locked_path_input_is_skipped_as_local_path` | — |
| returns nixpkgs input | 217 | ported | `nix.rs` | `extracts_nixpkgs_correctly` | — |
| includes nixpkgs with no explicit ref | 260 | ported | `nix.rs` | `includes_nixpkgs_with_no_explicit_ref` | — |
| includes patchelf from HEAD | 300 | ported | `nix.rs` | `includes_git_input_from_head` | — |
| includes ijq from sourcehut without a flake | 358 | ported | `nix.rs` | `includes_sourcehut_input_without_flake` | — |
| includes home-manager from gitlab | 399 | ported | `nix.rs` | `includes_gitlab_input` | — |
| test other version | 440 | ported | `nix.rs` | `other_lockfile_version_returns_empty` | — |
| includes nixpkgs with ref and shallow arguments | 452 | ported | `nix.rs` | `includes_git_input_with_ref_and_shallow_arguments` | — |
| includes nixpkgs but using indirect type that cannot be updated | 494 | ported | `nix.rs` | `original_indirect_input_is_skipped` | — |
| includes nixpkgs but using indirect type and path locked type that cannot be updated | 524 | ported | `nix.rs` | `original_indirect_locked_path_input_is_skipped_as_local_path` | — |
| includes flake from GitHub Enterprise | 553 | ported | `nix.rs` | `includes_github_enterprise_input` | — |
| includes flake with tarball type | 649 | ported | `nix.rs` | `includes_tarball_input_with_archive_url` | — |
| uri decode gitlab subgroup | 750 | ported | `nix.rs` | `decodes_gitlab_subgroup_owner` | — |
| includes flake with only tarball type | 790 | ported | `nix.rs` | `tarball_without_locked_rev_is_skipped_as_no_rev` | — |
| includes flake with nixpkgs-lib as tarball type | 818 | ported | `nix.rs` | `ignores_transitive_nixpkgs_lib_tarball_while_extracting_root_inputs` | — |
| includes flake with nixpkgs channel as tarball type | 897 | ported | `nix.rs` | `includes_nixpkgs_channel_tarball_input` | — |
| finds currentDigest correctly when input sha is pinned | 937 | ported | `nix.rs` | `extracts_current_digest_from_original_rev` | — |
| does not duplicate nixpkgs dependency | 983 | ported | `nix.rs` | `package_file_does_not_duplicate_nixpkgs_dependency` | — |
| returns null when flake.lock file cannot be read | 1028 | ported | `nix.rs` | `package_file_returns_none_when_flake_lock_missing` | — |
| returns null when flake.nix file cannot be read | 1033 | ported | `nix.rs` | `package_file_returns_none_when_flake_nix_missing` | — |
| returns null when flake.lock has invalid JSON | 1046 | ported | `nix.rs` | `invalid_json_returns_empty` | — |
| returns deps when no root inputs but deps exist | 1051 | ported | `nix.rs` | `root_without_inputs_returns_empty` | — |
| handles currentDigest replacement when config provided | 1065 | ported | `nix.rs` | `replaces_current_digest_when_config_matches_flake_nix` | — |
| includes nixpkgs with ref when original has rev | 1112 | ported | `nix.rs` | `includes_nixpkgs_ref_and_original_rev` | — |
| includes github flake with ref when original has rev | 1154 | ported | `nix.rs` | `includes_github_ref_and_original_rev` | — |
| includes gitlab flake with custom host | 1196 | ported | `nix.rs` | `includes_gitlab_input_with_custom_host` | — |
| includes sourcehut flake with custom host | 1238 | ported | `nix.rs` | `includes_sourcehut_input_with_custom_host` | — |
| includes tarball flake with ref when original has rev | 1280 | ported | `nix.rs` | `includes_tarball_input_ref_and_current_digest` | — |
| handles unknown flake lock type | 1321 | ported | `nix.rs` | `unknown_flake_lock_type_returns_empty` | — |
| ignores unsupported file type and still extracts other inputs | 1348 | ported | `nix.rs` | `unsupported_file_type_is_ignored_while_other_inputs_extract` | — |

---


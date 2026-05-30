# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mise/backends.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/backends.spec.ts
**Total tests:** 37 | **Ported:** 37 | **Actionable:** 0 | **Status:** done

### `createAquaToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config | 16 | ported | `mise.rs` | `aqua_create_tooling_config` | — |
| should trim the leading v from version | 27 | ported | `mise.rs` | `aqua_trim_leading_v` | — |

### `createCargoToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config for crate | 40 | ported | `mise.rs` | `cargo_create_crate_config` | — |
| should create a tooling config for git tag | 47 | ported | `mise.rs` | `cargo_create_git_tag_config` | — |
| should provide skipReason for git branch | 57 | ported | `mise.rs` | `cargo_create_git_branch_config` | — |
| should create a tooling config for git rev | 70 | ported | `mise.rs` | `cargo_create_git_rev_config` | — |
| should provide skipReason for invalid version | 80 | ported | `mise.rs` | `cargo_invalid_version_skip_reason` | — |

### `createDotnetToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config | 91 | ported | `mise.rs` | `dotnet_create_tooling_config` | — |

### `createGemToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config | 100 | ported | `mise.rs` | `gem_create_tooling_config` | — |

### `createGithubToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config with empty options | 109 | ported | `mise.rs` | `github_create_empty_options` | — |
| should not set extractVersion if the version has leading v | 119 | ported | `mise.rs` | `github_no_extract_version_with_v_prefix` | — |
| should set extractVersion with custom version_prefix | 127 | ported | `mise.rs` | `github_set_extract_version_with_prefix` | — |
| should set extractVersion with version_prefix even if version has leading v | 140 | ported | `mise.rs` | `github_extract_version_with_prefix_and_v_version` | — |
| should handle empty version_prefix with version not having v | 153 | ported | `mise.rs` | `github_empty_prefix_no_v` | — |
| should handle empty version_prefix with version having v | 163 | ported | `mise.rs` | `github_empty_prefix_with_v` | — |
| should escape special regex characters in version_prefix | 173 | ported | `mise.rs` | `github_escape_special_chars_in_prefix` | — |
| should escape brackets and parentheses in version_prefix | 186 | ported | `mise.rs` | `github_escape_brackets_in_prefix` | — |

### `createGoToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config | 201 | ported | `mise.rs` | `go_create_tooling_config` | — |

### `createNpmToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config | 210 | ported | `mise.rs` | `npm_create_tooling_config` | — |

### `createPipxToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config for pypi package | 219 | ported | `mise.rs` | `pipx_create_pypi_config` | — |
| should create a tooling config for github shorthand | 226 | ported | `mise.rs` | `pipx_create_github_shorthand_config` | — |
| should create a tooling config for github url | 233 | ported | `mise.rs` | `pipx_create_github_url_config` | — |
| should create a tooling config for git url | 242 | ported | `mise.rs` | `pipx_create_git_url_config` | — |
| provides skipReason for zip file url | 251 | ported | `mise.rs` | `pipx_zip_url_skip_reason` | — |

### `createSpmToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config for github shorthand | 262 | ported | `mise.rs` | `spm_create_github_shorthand_config` | — |
| should create a tooling config for github url | 269 | ported | `mise.rs` | `spm_create_github_url_config` | — |
| provides skipReason for other url | 278 | ported | `mise.rs` | `spm_non_github_url_skip_reason` | — |

### `createUbiToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a tooling config with empty options | 289 | ported | `mise.rs` | `ubi_create_empty_options` | — |
| should set extractVersion if the version does not have leading v | 298 | ported | `mise.rs` | `ubi_no_v_prefix_sets_extract_version` | — |
| should not set extractVersion if the version has leading v | 307 | ported | `mise.rs` | `ubi_v_prefix_no_extract_version` | — |
| should ignore options unless tag_regex is provided | 315 | ported | `mise.rs` | `ubi_ignore_options_without_tag_regex` | — |
| should set extractVersion if tag_regex is provided | 326 | ported | `mise.rs` | `ubi_set_extract_version_with_tag_regex` | — |
| should set extractVersion without v? when tag_regex is provided and version starts with v | 339 | ported | `mise.rs` | `ubi_no_v_opt_with_tag_regex_and_v_version` | — |
| should trim the leading ^ from tag_regex | 352 | ported | `mise.rs` | `ubi_trim_caret_from_tag_regex` | — |
| should only trim the leading ^ from tag_regex when version starts with v | 365 | ported | `mise.rs` | `ubi_trim_caret_v_prefix_keeps_v_in_regex` | — |
| should trim the leading ^v from tag_regex | 378 | ported | `mise.rs` | `ubi_trim_caret_v_from_tag_regex_no_v_version` | — |
| should trim the leading ^v? from tag_regex | 391 | ported | `mise.rs` | `ubi_trim_caret_v_opt_from_tag_regex` | — |

---


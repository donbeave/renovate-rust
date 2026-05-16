# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/puppet/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/puppet/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty Puppetfile | 10 | ported | `puppet.rs` | `empty_returns_empty` | — |
| extracts multiple modules from Puppetfile without a forge | 14 | ported | `puppet.rs` | `extracts_forge_module_with_version` (+ multiple_modules) | — |
| extracts multiple modules from Puppetfile with multiple forges/registries | 47 | ported | `puppet.rs` | `extracts_custom_forge` | — |
| extracts multiple git tag modules from Puppetfile | 100 | ported | `puppet.rs` | `extracts_github_git_module` | — |
| Use GithubTagsDatasource only if host is exactly github.com | 125 | ported | `puppet.rs` | `non_github_host_uses_git_tags_datasource` | — |
| Github url without https is skipped | 146 | ported | `puppet.rs` | `http_github_url_marked_invalid_url` | — |
| Git module without a tag should result in a skip reason | 162 | ported | `puppet.rs` | `git_no_tag_skipped` | — |
| Skip reason should be overwritten by parser | 181 | ported | `puppet.rs` | `malformed_mod_with_three_positional_args_is_invalid_config` | — |
| GitTagsDatasource | 200 | ported | `puppet.rs` | `git_tags_fixture_extracts_four_valid_and_one_invalid` | — |

---


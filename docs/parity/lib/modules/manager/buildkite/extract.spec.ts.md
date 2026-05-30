# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/buildkite/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/buildkite/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 7 | ported | `buildkite.rs` | `empty_content_returns_no_deps` | — |
| extracts simple single plugin | 11 | ported | `buildkite.rs` | `two_part_plugin` | — |
| extracts multiple plugins in same file | 22 | ported | `buildkite.rs` | `one_part_plugin` | — |
| adds skipReason | 47 | ported | `buildkite.rs` | `non_semver_version_skipped` | — |
| extracts arrays of plugins | 70 | ported | `buildkite.rs` | `array_plugins_extracted` | — |
| extracts git-based plugins | 92 | ported | `buildkite.rs` | `github_url_plugin` | — |
| extracts git-based plugin with .git at the end of its name | 105 | ported | `buildkite.rs` | `github_enterprise_ssh_url_with_git_suffix` | — |
| extracts plugins outside plugins sections | 121 | ported | `buildkite.rs` | `ssh_url_plugin_outside_plugins_section` | — |
| extracts plugin with preceding ? | 140 | ported | `buildkite.rs` | `yaml_question_mark_prefix` | — |
| extracts plugin tags from bitbucket | 155 | ported | `buildkite.rs` | `bitbucket_plugin_extracted` | — |
| extracts plugin tags with quotes | 178 | ported | `buildkite.rs` | `single_quoted_plugin` | — |

---


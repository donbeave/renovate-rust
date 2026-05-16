# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/copier/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/copier/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts repository and version from .copier-answers.yml | 6 | ported | `copier.rs` | `extracts_github_url` | — |
| extracts repository and version from .copier-answers.yml with ssh URL | 25 | ported | `copier.rs` | `ssh_url_src_path_extracted` | — |
| extracts repository and version from .copier-answers.yml with ssh URL and non-bare Repo | 44 | ported | `copier.rs` | `non_github_ssh_url_extracted` | — |
| extracts repository and version from .copier-answers.yml with ssh URL and a username different from git | 63 | ported | `copier.rs` | `non_git_username_ssh_url_extracted` | — |
| extracts and strips git+ prefix from $srcPath | 84 | ported | `copier.rs` | `strips_git_plus_prefix` | — |
| returns null for invalid .copier-answers.yml | 119 | ported | `copier.rs` | `invalid_yaml_returns_none` | — |
| returns null for invalid _src_path | 128 | ported | `copier.rs` | `non_url_src_path_extracted_without_github_repo` | — |
| returns null for missing _commit field | 137 | ported | `copier.rs` | `missing_commit_returns_none` | — |
| returns null for missing _src_path field | 145 | ported | `copier.rs` | `missing_src_path_returns_none` | — |

---


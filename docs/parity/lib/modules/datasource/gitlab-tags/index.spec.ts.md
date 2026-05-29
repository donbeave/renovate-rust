# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gitlab-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitlab-tags/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/datasource/gitlab-tags/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from custom registry | 9 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `returns_tags_from_custom_registry` | custom registry URL → 3 tags |
| returns tags from custom registry in sub path | 38 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `returns_tags_from_custom_registry_in_sub_path` | subpath registry URL → dep_host strips /api/v4 |
| returns tags with default registry | 67 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `returns_tags_with_default_registry` | no registryUrl → gitlab.com default |

### `modules/datasource/gitlab-tags/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commits from gitlab installation | 83 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `returns_commits_from_gitlab_installation` | tag → commit hash |
| returns commits from gitlab installation for a specific branch | 102 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `returns_commits_for_specific_branch` | newValue branch → commit hash |
| returns null from gitlab installation with no commits | 122 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `returns_null_with_no_commits` | empty commits array → None |
| returns null from gitlab installation with unknown branch | 135 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `returns_null_for_unknown_branch` | 404 → None |

---

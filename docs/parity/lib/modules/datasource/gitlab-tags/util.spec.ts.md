# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gitlab-tags/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitlab-tags/util.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `modules/datasource/gitlab-tags/util › getDepHost`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 5 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `get_dep_host_works` | strips /api/v4 suffix; default = gitlab.com |

### `modules/datasource/gitlab-tags/util › getSourceUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 17 | ported | `crates/renovate-core/src/datasources/gitlab_tags.rs` | `get_source_url_works` | depHost + packageName; default registry |

---

# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/gitlab-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitlab-tags/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/gitlab-tags/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from custom registry | 9 | not-applicable | — | — | Renovate's GitLab tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup. |
| returns tags from custom registry in sub path | 38 | not-applicable | — | — | Renovate's GitLab tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup. |
| returns tags with default registry | 67 | not-applicable | — | — | Renovate's GitLab tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup. |

### `modules/datasource/gitlab-tags/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commits from gitlab installation | 83 | not-applicable | — | — | Renovate's GitLab tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup. |
| returns commits from gitlab installation for a specific branch | 102 | not-applicable | — | — | Renovate's GitLab tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup. |
| returns null from gitlab installation with no commits | 122 | not-applicable | — | — | Renovate's GitLab tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup. |
| returns null from gitlab installation with unknown branch | 135 | not-applicable | — | — | Renovate's GitLab tags release-list and digest APIs are not implemented in Rust; Rust only exposes latest tag lookup. |

---


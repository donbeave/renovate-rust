# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/releases-direct.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/releases-direct.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/go/releases-direct › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for null getDatasource result | 26 | not-applicable | — | — | Renovate's Go direct datasource discovery and delegation to tag datasources are not implemented in Rust; Rust Go support queries a supplied Go proxy `@latest` endpoint. |
| throws for getDatasource error | 34 | not-applicable | — | — | Renovate's Go direct datasource discovery error path is not implemented in Rust. |
| processes real data | 43 | not-applicable | — | — | Renovate's direct GitHub tag release-list delegation is not implemented in Rust. |
| support forgejo | 69 | not-applicable | — | — | Renovate's direct Forgejo tag release-list delegation is not implemented in Rust. |
| support gitlab | 130 | not-applicable | — | — | Renovate's direct GitLab tag release-list delegation is not implemented in Rust. |
| support gitea | 148 | not-applicable | — | — | Renovate's direct Gitea tag release-list delegation is not implemented in Rust. |
| support git | 209 | not-applicable | — | — | Renovate's direct generic git tag release-list delegation is not implemented in Rust. |
| support self hosted gitlab private repositories | 228 | not-applicable | — | — | Renovate's hostRules-authenticated self-hosted GitLab tag release-list delegation is not implemented in Rust. |
| support bitbucket tags | 247 | not-applicable | — | — | Renovate's direct Bitbucket tag release-list delegation is not implemented in Rust. |
| support ghe | 269 | not-applicable | — | — | Renovate's GitHub Enterprise tag release-list delegation is not implemented in Rust. |
| works for known servers | 298 | not-applicable | — | — | Renovate's known Go import-host to tag-datasource mapping is not implemented in Rust. |
| support gitlab subgroups | 327 | not-applicable | — | — | Renovate's GitLab subgroup tag release-list delegation is not implemented in Rust. |
| works for nested modules on github | 347 | not-applicable | — | — | Renovate's nested Go module tag-prefix filtering for direct GitHub releases is not implemented in Rust. |
| falls back to unprefixed tags | 383 | not-applicable | — | — | Renovate's nested Go module fallback to unprefixed tags is not implemented in Rust. |
| works for nested modules on github v2+ major upgrades | 409 | not-applicable | — | — | Renovate's nested Go module v2+ tag-prefix filtering for direct GitHub releases is not implemented in Rust. |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/go/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches releases | 68 | not-applicable | — | — | Renovate's Go datasource orchestration between Go proxy and direct datasource release-list providers is not implemented in Rust; Rust directly queries a supplied Go proxy `@latest` endpoint. |

### `modules/datasource/go/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for no go-source tag | 89 | not-applicable | — | — | Renovate's Go `go-get=1` HTML meta tag parsing and digest delegation are not implemented in Rust. |
| returns null for wrong name | 101 | not-applicable | — | — | Renovate's Go `go-get=1` HTML meta tag validation and digest delegation are not implemented in Rust. |
| supports gitlab digest | 113 | not-applicable | — | — | Renovate's Go digest delegation to GitLab tags is not implemented in Rust. |
| supports git digest | 126 | not-applicable | — | — | Renovate's Go digest delegation to generic git tags is not implemented in Rust. |
| supports gitlab digest with a specific branch | 139 | not-applicable | — | — | Renovate's Go digest delegation with branch handling is not implemented in Rust. |
| returns github digest | 153 | not-applicable | — | — | Renovate's Go digest delegation to GitHub tags is not implemented in Rust. |
| returns github default branch digest | 174 | not-applicable | — | — | Renovate's Go digest delegation to GitHub default branch is not implemented in Rust. |
| support bitbucket digest | 195 | not-applicable | — | — | Renovate's Go digest delegation to Bitbucket tags is not implemented in Rust. |
| support forgejo digest | 206 | not-applicable | — | — | Renovate's Go digest delegation to Forgejo tags is not implemented in Rust. |
| support gitea digest | 217 | not-applicable | — | — | Renovate's Go digest delegation to Gitea tags is not implemented in Rust. |

### `modules/datasource/go/index › getDigest › GOPROXY`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when GOPROXY contains off | 233 | not-applicable | — | — | Renovate's GOPROXY parser and digest-source suppression are not implemented in Rust. |

### `modules/datasource/go/index › using getPkgReleases › constraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are respected based on an exact match on the `go` constraint | 256 | not-applicable | — | — | Renovate's datasource-level release-list constraint filtering is not implemented in Rust; Rust Go datasource returns a latest-version update summary. |
| are respected based on a SemVer-style range based on the `%goMod` constraint | 298 | not-applicable | — | — | Renovate's datasource-level release-list constraint filtering with `%goMod` versioning is not implemented in Rust; Rust Go datasource returns a latest-version update summary. |

---


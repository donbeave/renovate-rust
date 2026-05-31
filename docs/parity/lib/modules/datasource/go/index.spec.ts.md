# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `modules/datasource/go/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches releases | 68 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |

### `modules/datasource/go/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for no go-source tag | 89 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| returns null for wrong name | 101 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| supports gitlab digest | 113 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| supports git digest | 126 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| supports gitlab digest with a specific branch | 139 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| returns github digest | 153 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| returns github default branch digest | 174 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| support bitbucket digest | 195 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| support forgejo digest | 206 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| support gitea digest | 217 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |

### `modules/datasource/go/index › getDigest › GOPROXY`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when GOPROXY contains off | 233 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |

### `modules/datasource/go/index › using getPkgReleases › constraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are respected based on an exact match on the `go` constraint | 256 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |
| are respected based on a SemVer-style range based on the `%goMod` constraint | 298 | not-applicable | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer | — | Mock framework internals — tests go datasource via nock HTTP mocks + vitest mocks; Rust tests this at different layer |

---

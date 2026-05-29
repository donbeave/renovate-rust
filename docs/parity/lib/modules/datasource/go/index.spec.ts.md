# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** pending

### `modules/datasource/go/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches releases | 68 | pending | — | — | —|

### `modules/datasource/go/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for no go-source tag | 89 | pending | — | — | —|
| returns null for wrong name | 101 | pending | — | — | —|
| supports gitlab digest | 113 | pending | — | — | —|
| supports git digest | 126 | pending | — | — | —|
| supports gitlab digest with a specific branch | 139 | pending | — | — | —|
| returns github digest | 153 | pending | — | — | —|
| returns github default branch digest | 174 | pending | — | — | —|
| support bitbucket digest | 195 | pending | — | — | —|
| support forgejo digest | 206 | pending | — | — | —|
| support gitea digest | 217 | pending | — | — | —|

### `modules/datasource/go/index › getDigest › GOPROXY`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when GOPROXY contains off | 233 | pending | — | — | —|

### `modules/datasource/go/index › using getPkgReleases › constraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are respected based on an exact match on the `go` constraint | 256 | pending | — | — | —|
| are respected based on a SemVer-style range based on the `%goMod` constraint | 298 | pending | — | — | —|

---

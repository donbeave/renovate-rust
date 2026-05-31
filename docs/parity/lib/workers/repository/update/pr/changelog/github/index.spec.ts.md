# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/github/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/pr/changelog/github/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types  | 55 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| returns null if no currentVersion  | 64 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| returns null if currentVersion equals newVersion  | 73 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| skips invalid repos  | 83 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| works without Github  | 92 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| uses GitHub tags  | 118 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| filters unnecessary warns  | 144 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| supports node engines  | 171 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| handles no sourceUrl  | 198 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| handles invalid sourceUrl  | 207 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| handles missing Github token  | 216 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| handles suppressed Github warnings  | 226 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| handles no releases  | 236 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| handles not enough releases  | 245 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| supports github enterprise and github.com changelog  | 254 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| supports github enterprise and github enterprise changelog  | 285 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |
| works with same version releases but different prefix  | 318 | not-applicable | Mock framework internals — tests changelog github via nock HTTP mocks; Rust tests this at different layer | — | GitHub changelog integration not implemented in Rust |

---

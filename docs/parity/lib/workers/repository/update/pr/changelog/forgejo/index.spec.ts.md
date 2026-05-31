# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/forgejo/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/pr/changelog/forgejo/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types  | 56 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| returns null if currentVersion equals newVersion  | 65 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| skips invalid repos  | 75 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| works without forgejo  | 84 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| uses forgejo tags  | 111 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| handles empty forgejo tags response  | 224 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| uses forgejo tags with error  | 259 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| handles no sourceUrl  | 294 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| handles invalid sourceUrl  | 303 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| handles no releases  | 312 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| handles not enough releases  | 321 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| supports self-hosted forgejo changelog  | 330 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |

### `workers/repository/update/pr/changelog/forgejo/index › hasValidRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles invalid repository  | 367 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |
| handles valid repository  | 372 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |

### `workers/repository/update/pr/changelog/forgejo/index › getAllTags`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles endpoint  | 378 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |

### `workers/repository/update/pr/changelog/forgejo/index › getReleaseNotesMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works  | 394 | not-applicable | Mock framework internals — tests changelog forgejo via nock HTTP mocks; Rust tests this at different layer | — | Forgejo changelog integration not implemented in Rust |

---


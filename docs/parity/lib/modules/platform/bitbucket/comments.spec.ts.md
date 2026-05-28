# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket/comments.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket/comments.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** done

### `ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 15 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| add comment if not found | 31 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| finds reopen comment | 50 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| finds no reopen comment | 73 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| add updates comment if necessary | 96 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| skips comment | 120 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `ensureCommentRemoval()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 145 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| deletes comment by topic if found | 160 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| deletes comment by content if found | 185 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| deletes nothing | 210 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---


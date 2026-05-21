# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/gitea/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/gitea/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 17 | **Status:** not-applicable

### `workers/repository/update/pr/changelog/gitea/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 56 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| returns null if currentVersion equals newVersion | 65 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| skips invalid repos | 75 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| works without gitea | 84 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| uses gitea tags | 111 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| handles empty gitea tags response | 224 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| uses gitea tags with error | 259 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| handles no sourceUrl | 294 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| handles invalid sourceUrl | 303 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| handles no releases | 312 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| handles not enough releases | 321 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| supports gitea enterprise and gitea enterprise changelog | 330 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| supports self-hosted gitea changelog | 364 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |

### `workers/repository/update/pr/changelog/gitea/index › hasValidRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles invalid repository | 401 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |
| handles valid repository | 406 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |

### `workers/repository/update/pr/changelog/gitea/index › getAllTags`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles endpoint | 412 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |

### `workers/repository/update/pr/changelog/gitea/index › getReleaseNotesMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 428 | not-applicable | — | — | tests Gitea changelog fetching via HTTP API; platform API calls out of scope |

---


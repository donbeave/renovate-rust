# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/pr/changelog/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 56 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| handles unsupported changelog source | 65 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| returns null if no currentVersion | 74 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| returns null if currentVersion equals newVersion | 83 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| skips invalid repos | 93 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| works without Github | 102 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| uses GitHub tags | 140 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| filters unnecessary warns | 176 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| supports node engines | 206 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| handles no sourceUrl | 236 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| handles invalid sourceUrl | 245 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| handles missing Github token | 254 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| handles no releases | 264 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| handles not enough releases | 273 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| will call getInRangeReleases when releases is undefined | 282 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| supports github enterprise and github.com changelog | 291 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| supports github enterprise and github enterprise changelog | 325 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |
| supports github.com and github enterprise changelog | 364 | not-applicable | — | — | tests changelog fetching via platform API + GitHub/GitLab HTTP; platform API calls out of scope |

---

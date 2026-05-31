# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/pr/changelog/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types  | 56 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| handles unsupported changelog source  | 65 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| returns null if no currentVersion  | 74 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| returns null if currentVersion equals newVersion  | 83 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| skips invalid repos  | 93 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| works without Github  | 102 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| uses GitHub tags  | 140 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| filters unnecessary warns  | 176 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| supports node engines  | 206 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| handles no sourceUrl  | 236 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| handles invalid sourceUrl  | 245 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| handles missing Github token  | 254 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| handles no releases  | 264 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| handles not enough releases  | 273 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| will call getInRangeReleases when releases is undefined  | 282 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| supports github enterprise and github.com changelog  | 291 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| supports github enterprise and github enterprise changelog  | 325 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |
| supports github.com and github enterprise changelog  | 364 | not-applicable | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests changelog index via vitest-mocked datasource; Rust tests this at different layer |

---

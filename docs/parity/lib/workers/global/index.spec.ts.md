# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/global/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/index.spec.ts
**Total tests:** 15 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `workers/global/index › getRepositoryConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should generate correct topLevelOrg/parentOrg with multiple levels | 56 | ported | `util.rs` | `parse_repo_org_multiple_levels` | — |
| should generate correct topLevelOrg/parentOrg with two levels | 67 | ported | `util.rs` | `parse_repo_org_two_levels` | — |
| stores repositoryEntryConfig for repositories[] object entries  | 78 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| does not store repositoryEntryConfig for repositories[] string entries  | 91 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| handles config warnings and errors  | 101 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| handles zero repos  | 114 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| handles local  | 125 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| processes repositories  | 134 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| processes repositories break  | 152 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| exits with non-zero when errors are logged  | 173 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| exits with zero when warnings are logged  | 189 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| does not log info message when log level is not info  | 206 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |

### `workers/global/index › processes platforms`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| github  | 220 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |
| gitlab  | 231 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |

### `workers/global/index › write repositories to file`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| successfully write file  | 244 | not-applicable | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer | — | Mock framework internals — tests global worker via vitest-mocked platform/config/logger/fs; Rust tests this at different layer |

---


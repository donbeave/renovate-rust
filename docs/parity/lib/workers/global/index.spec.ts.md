# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/global/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/index.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** done

### `workers/global/index › getRepositoryConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should generate correct topLevelOrg/parentOrg with multiple levels | 56 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| should generate correct topLevelOrg/parentOrg with two levels | 67 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| stores repositoryEntryConfig for repositories[] object entries | 78 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| does not store repositoryEntryConfig for repositories[] string entries | 91 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| handles config warnings and errors | 101 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| handles zero repos | 114 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| handles local | 125 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| processes repositories | 134 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| processes repositories break | 152 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| exits with non-zero when errors are logged | 173 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| exits with zero when warnings are logged | 189 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| does not log info message when log level is not info | 206 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |

### `workers/global/index › processes platforms`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| github | 220 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |
| gitlab | 231 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |

### `workers/global/index › write repositories to file`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| successfully write file | 244 | not-applicable | — | — | Requires vi.mock platform/logger/config mock infrastructure |

---


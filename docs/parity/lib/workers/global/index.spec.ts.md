# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/global/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/index.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** not-applicable

### `workers/global/index › getRepositoryConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should generate correct topLevelOrg/parentOrg with multiple levels | 56 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| should generate correct topLevelOrg/parentOrg with two levels | 67 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| stores repositoryEntryConfig for repositories[] object entries | 78 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| does not store repositoryEntryConfig for repositories[] string entries | 91 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| handles config warnings and errors | 101 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| handles zero repos | 114 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| handles local | 125 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| processes repositories | 134 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| processes repositories break | 152 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| exits with non-zero when errors are logged | 173 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| exits with zero when warnings are logged | 189 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| does not log info message when log level is not info | 206 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |

### `workers/global/index › processes platforms`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| github | 220 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |
| gitlab | 231 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |

### `workers/global/index › write repositories to file`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| successfully write file | 244 | not-applicable | — | — | tests global worker orchestration entry point; CLI-level orchestration out of scope |

---


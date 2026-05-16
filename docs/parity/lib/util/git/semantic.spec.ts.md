# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/semantic.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/semantic.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `util/git/semantic › detectSemanticCommits()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects false if unknown | 18 | ported | `branch.rs` | `semantic_commits_disabled_for_non_semantic` | Tests inner score logic; git/cache mocking not applicable |
| detects true if known | 31 | ported | `branch.rs` | `semantic_commits_enabled_for_semantic` | — |
| detects false on malformed commits | 38 | ported | `branch.rs` | `semantic_commits_disabled_for_malformed` | — |
| detects true on breaking changes | 49 | ported | `branch.rs` | `semantic_commits_enabled_for_breaking_changes` | — |
| detects true on breaking changes with scope | 56 | ported | `branch.rs` | `semantic_commits_enabled_for_breaking_changes_with_scope` | — |

---


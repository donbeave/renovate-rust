# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/commit-message.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/commit-message.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `workers/repository/config-migration/branch/commit-message`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates semantic commit message | 8 | ported | `branch.rs` | `config_migration_semantic_commit_message` | — |
| creates semantic pr title | 19 | ported | `branch.rs` | `config_migration_semantic_pr_title` | — |
| creates non-semantic commit message | 30 | ported | `branch.rs` | `config_migration_non_semantic_commit_message` | — |
| creates non-semantic pr title | 41 | ported | `branch.rs` | `config_migration_non_semantic_pr_title` | — |
| returns default values when commitMessage template string is empty | 50 | ported | `branch.rs` | `config_migration_pr_title_with_empty_commit_message` | commitMessage='' falls back to default title |

---


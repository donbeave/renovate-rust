# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/hermit/default-config.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/hermit/default-config.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `excludeCommitPaths`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| minimatches("$path") === $expected | 13 | ported | `hermit.rs` | `hermit_exclude_commit_paths_glob` | — |

### `managerFilePatterns`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matchRegexOrGlobList("$path") === $expected | 30 | ported | `hermit.rs` | `hermit_file_pattern_matches_expected` | — |

---


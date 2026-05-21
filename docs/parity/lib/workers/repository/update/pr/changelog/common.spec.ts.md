# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/common.spec.ts
**Total tests:** 2 | **Ported:** 1 | **Actionable:** 2 | **Status:** partial

### `workers/repository/update/pr/changelog/common › slugifyUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| slugifyUrl("$url") === $expected | 5 | not-applicable | — | — | tests changelog source selection logic tied to TypeScript platform integration |

### `workers/repository/update/pr/changelog/common › compareChangelogFilePath()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sorts $files to $expected | 18 | ported | `branch.rs` | `compare_changelog_file_path_sorts_by_type_preference` | — |

---


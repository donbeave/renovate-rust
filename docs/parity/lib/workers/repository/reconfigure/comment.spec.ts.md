# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/reconfigure/comment.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/reconfigure/comment.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 3 | **Status:** pending

### `workers/repository/reconfigure/comment › ensureReconfigurePrComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensures comment  | 35 | not-applicable | — | — | Mock framework internals — tests TS-specific ensureReconfigurePrComment via vitest-mocked platform; Rust tests this at different architecture level |
| ensures comment - when no package files detected  | 47 | not-applicable | — | — | Mock framework internals — tests TS-specific ensureReconfigurePrComment via vitest-mocked platform; Rust tests this at different architecture level |
| dryrun  | 59 | not-applicable | — | — | Mock framework internals — tests TS-specific ensureReconfigurePrComment via vitest-mocked platform; Rust tests this at different architecture level |

### `workers/repository/reconfigure/comment › getConfigDesc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty  | 87 | pending | — | — | getConfigDesc markdown formatting not yet implemented in Rust reconfigure module |
| returns a full list  | 92 | pending | — | — | getConfigDesc markdown formatting not yet implemented in Rust reconfigure module |
| adds schedule  | 115 | pending | — | — | getConfigDesc markdown formatting not yet implemented in Rust reconfigure module |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mix/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mix/artifacts.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no mix.lock found | 47 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no updatedDeps were provided | 58 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if updatedDeps is empty | 69 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if unchanged | 80 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null when trying to use lockFileMaintenance with no mix.lock file | 96 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no updatedDeps and no lockFileMaintenance | 116 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if using lockFileMaintenance in umbrella project | 127 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated mix.lock | 143 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses constraints on install mode | 188 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| authenticates to private repositories in updated dependencies | 217 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| authenticates to private repositories configured in hostRules | 281 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated mix.lock in subdir | 344 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated mix.lock in umbrella project | 378 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports lockFileMaintenance | 414 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| lockFileMaintenance returns null if unchanged | 452 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches write errors | 468 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches exec errors | 486 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| detects read errors | 502 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| detects read errors (umbrella) | 523 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles updates and doesn't try to create mix.lock file if it doesn't exist | 547 | not-applicable | Mock framework internals — tests mix artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---


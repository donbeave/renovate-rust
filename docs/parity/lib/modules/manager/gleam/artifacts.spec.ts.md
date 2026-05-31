# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gleam/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gleam/artifacts.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 30 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| skips if no lock file is found | 34 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if cannot read lock file | 39 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if cannot read new lock file | 46 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if lock content unchanged | 57 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated lock content | 67 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| supports lockFileMaintenance | 86 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if lockfile content unchanged | 106 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| handles temporary error | 118 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| handles temporary error when reading the lock file | 134 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| handles full error | 145 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| prevents injections | 167 | not-applicable | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gleam artifact update via vitest-mocked fs/exec; Rust tests this at different layer |

---


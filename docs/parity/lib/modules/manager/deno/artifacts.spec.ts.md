# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/artifacts.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 38 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| skips if no lock file in config | 42 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| skips and returns an error if cannot read lock file | 47 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| returns null if lock content unchanged | 59 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| returns updated lock content | 68 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| change directory if import map is used | 86 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| supports lockFileMaintenance | 111 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| handles temporary error | 130 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| handles full error | 146 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| depType tasks returns an error | 163 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| depType tasks.command returns an error | 187 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| supports lockFileMaintenance (without updated deps) | 211 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |
| deno command execution | 236 | not-applicable | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer | — | Mock framework internals — tests deno artifacts via vitest-mocked fs/exec + tmp-promise; Rust tests this at different layer |

---


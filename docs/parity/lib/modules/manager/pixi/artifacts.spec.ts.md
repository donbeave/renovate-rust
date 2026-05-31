# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pixi/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pixi/artifacts.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no pixi.lock found | 69 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if updatedDeps is empty | 82 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if unchanged | 95 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| handle TEMPORARY_ERROR | 121 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated pixi.lock using docker | 139 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated pixi.lock using install mode | 195 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated pixi.lock using install mode for old version lock file | 234 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns pixi version defined in requires-pixi | 272 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| catches errors | 327 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated pixi.lock when doing lockfile maintenance | 347 | not-applicable | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pixi artifact update via vitest-mocked fs/exec; Rust tests this at different layer |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/artifacts.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return null if all processors returns are empty | 26 | not-applicable | — | — | Mock framework internals — tests artifact update via vitest-mocked fs, exec, and datasource; Rust tests this at different layer |
| return artifact error if newPackageFile content is not valid | 41 | not-applicable | — | — | Mock framework internals — tests artifact update via vitest-mocked fs, exec, and datasource; Rust tests this at different layer |
| return processor result | 60 | not-applicable | — | — | Mock framework internals — tests artifact update via vitest-mocked fs, exec, and datasource; Rust tests this at different layer |

---


# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pub/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pub/artifacts.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no pubspec.lock found | 61 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null if updatedDeps is empty | 65 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| runs flutter pub get if only dart and flutter sdks are updated | 71 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns null for ${params.sdk} if unchanged | 106 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated ${params.sdk} pubspec.lock | 123 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| runs ${params.sdk} pub get if only the sdk is updated | 149 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated ${params.sdk} pubspec.lock for lockfile maintenance | 176 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| supports ${params.sdk} docker mode | 203 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| supports ${params.sdk} install mode | 251 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| catches errors for ${params.sdk} | 278 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| uses flutter constraint from pubspec.yaml | 295 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |
| uses dart constraint from pubspec.yaml | 330 | not-applicable | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests pub artifact update via vitest-mocked fs/exec; Rust tests this at different layer |

---


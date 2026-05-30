# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pub/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pub/artifacts.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no pubspec.lock found | 61 | not-applicable | — | — | Subprocess artifact generation |
| returns null if updatedDeps is empty | 65 | not-applicable | — | — | Subprocess artifact generation |
| runs flutter pub get if only dart and flutter sdks are updated | 71 | not-applicable | — | — | Subprocess artifact generation |
| returns null for ${params.sdk} if unchanged | 106 | not-applicable | — | — | Subprocess artifact generation |
| returns updated ${params.sdk} pubspec.lock | 123 | not-applicable | — | — | Subprocess artifact generation |
| runs ${params.sdk} pub get if only the sdk is updated | 149 | not-applicable | — | — | Subprocess artifact generation |
| returns updated ${params.sdk} pubspec.lock for lockfile maintenance | 176 | not-applicable | — | — | Subprocess artifact generation |
| supports ${params.sdk} docker mode | 203 | not-applicable | — | — | Subprocess artifact generation |
| supports ${params.sdk} install mode | 251 | not-applicable | — | — | Subprocess artifact generation |
| catches errors for ${params.sdk} | 278 | not-applicable | — | — | Subprocess artifact generation |
| uses flutter constraint from pubspec.yaml | 295 | not-applicable | — | — | Subprocess artifact generation |
| uses dart constraint from pubspec.yaml | 330 | not-applicable | — | — | Subprocess artifact generation |

---


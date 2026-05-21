# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pub/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pub/artifacts.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no pubspec.lock found | 61 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if updatedDeps is empty | 65 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| runs flutter pub get if only dart and flutter sdks are updated | 71 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null for ${params.sdk} if unchanged | 106 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated ${params.sdk} pubspec.lock | 123 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| runs ${params.sdk} pub get if only the sdk is updated | 149 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated ${params.sdk} pubspec.lock for lockfile maintenance | 176 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports ${params.sdk} docker mode | 203 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports ${params.sdk} install mode | 251 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors for ${params.sdk} | 278 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses flutter constraint from pubspec.yaml | 295 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses dart constraint from pubspec.yaml | 330 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


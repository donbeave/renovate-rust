# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/npm/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/artifacts.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no packageManager updates present | 56 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if currentValue is undefined | 67 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if currentValue has no hash | 78 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if unchanged | 89 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated package.json | 104 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode | 130 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports install mode | 179 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors | 220 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `updatePnpmWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no security updates are found | 242 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if pnpm workspace file does not exist | 253 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if the pnpmShrinkwrap file is not found | 273 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no minimumReleaseAge setting found | 304 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if minimumReleaseAgeExclude excludes all versions of updated dep | 325 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates pnpm workspace - adds minimumReleaseAgeExclude block if not found | 358 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates pnpm workspace - appends new minimumReleaseAgeExclude setting | 389 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates pnpm workspace - expands existing minimumReleaseAgeExclude setting | 422 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates pnpm workspace - handles comment with version already present on an inner minimumReleaseAgeExclude setting | 465 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates pnpm workspace - handles comment on an inner minimumReleaseAgeExclude setting | 496 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates pnpm workspace - uses newVersion over newValue in minimumReleaseAgeExclude | 536 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles multiple security upgrades of the same package (at different versions) in a monorepo | 572 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles multiple security upgrades of the same package (at the same version) in a monorepo | 643 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| preserves catalog changes in pnpm-workspace.yaml when adding minimumReleaseAgeExclude | 706 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles multiple security upgrades correctly (bug fix test) | 746 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---


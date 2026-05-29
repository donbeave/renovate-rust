# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/npm/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/artifacts.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 23 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no packageManager updates present | 56 | pending | — | — | —|
| returns null if currentValue is undefined | 67 | pending | — | — | —|
| returns null if currentValue has no hash | 78 | pending | — | — | —|
| returns null if unchanged | 89 | pending | — | — | —|
| returns updated package.json | 104 | pending | — | — | —|
| supports docker mode | 130 | pending | — | — | —|
| supports install mode | 179 | pending | — | — | —|
| catches errors | 220 | pending | — | — | —|

### `updatePnpmWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no security updates are found | 242 | pending | — | — | —|
| returns null if pnpm workspace file does not exist | 253 | pending | — | — | —|
| returns null if the pnpmShrinkwrap file is not found | 273 | pending | — | — | —|
| returns null if no minimumReleaseAge setting found | 304 | pending | — | — | —|
| returns null if minimumReleaseAgeExclude excludes all versions of updated dep | 325 | pending | — | — | —|
| updates pnpm workspace - adds minimumReleaseAgeExclude block if not found | 358 | pending | — | — | —|
| updates pnpm workspace - appends new minimumReleaseAgeExclude setting | 389 | pending | — | — | —|
| updates pnpm workspace - expands existing minimumReleaseAgeExclude setting | 422 | pending | — | — | —|
| updates pnpm workspace - handles comment with version already present on an inner minimumReleaseAgeExclude setting | 465 | pending | — | — | —|
| updates pnpm workspace - handles comment on an inner minimumReleaseAgeExclude setting | 496 | pending | — | — | —|
| updates pnpm workspace - uses newVersion over newValue in minimumReleaseAgeExclude | 536 | pending | — | — | —|
| handles multiple security upgrades of the same package (at different versions) in a monorepo | 572 | pending | — | — | —|
| handles multiple security upgrades of the same package (at the same version) in a monorepo | 643 | pending | — | — | —|
| preserves catalog changes in pnpm-workspace.yaml when adding minimumReleaseAgeExclude | 706 | pending | — | — | —|
| handles multiple security upgrades correctly (bug fix test) | 746 | pending | — | — | —|

---


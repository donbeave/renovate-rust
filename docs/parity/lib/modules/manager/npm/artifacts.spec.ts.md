# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/npm/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/artifacts.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no packageManager updates present | 56 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if currentValue is undefined | 67 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if currentValue has no hash | 78 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if unchanged | 89 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated package.json | 104 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports docker mode | 130 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports install mode | 179 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 220 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `updatePnpmWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no security updates are found | 242 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if pnpm workspace file does not exist | 253 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if the pnpmShrinkwrap file is not found | 273 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no minimumReleaseAge setting found | 304 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if minimumReleaseAgeExclude excludes all versions of updated dep | 325 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates pnpm workspace - adds minimumReleaseAgeExclude block if not found | 358 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates pnpm workspace - appends new minimumReleaseAgeExclude setting | 389 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates pnpm workspace - expands existing minimumReleaseAgeExclude setting | 422 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates pnpm workspace - handles comment with version already present on an inner minimumReleaseAgeExclude setting | 465 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates pnpm workspace - handles comment on an inner minimumReleaseAgeExclude setting | 496 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates pnpm workspace - uses newVersion over newValue in minimumReleaseAgeExclude | 536 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles multiple security upgrades of the same package (at different versions) in a monorepo | 572 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles multiple security upgrades of the same package (at the same version) in a monorepo | 643 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| preserves catalog changes in pnpm-workspace.yaml when adding minimumReleaseAgeExclude | 706 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles multiple security upgrades correctly (bug fix test) | 746 | not-applicable | Mock framework internals — tests npm artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---


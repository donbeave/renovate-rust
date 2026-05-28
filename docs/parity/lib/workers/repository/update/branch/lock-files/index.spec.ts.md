# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/lock-files/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/lock-files/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `workers/repository/update/branch/lock-files/index › writeUpdatedPackageFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no updated packageFiles | 30 | not-applicable | — | — | Uses vi.mock(fs) and fs.writeLocalFile mock; not portable |
| returns if no updated packageFiles are package.json | 36 | not-applicable | — | — | Uses vi.mock(fs) and fs.writeLocalFile mock; not portable |
| writes updated packageFiles | 48 | not-applicable | — | — | Uses vi.mock(fs) and fs.writeLocalFile mock; not portable |

### `workers/repository/update/branch/lock-files/index › getAdditionalFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns no error and empty lockfiles if skipArtifactsUpdate is true | 90 | not-applicable | — | — | Uses vi.spyOn(npm/yarn/pnpm.generateLockFile) and git.getFile mock; not portable |
| returns no error and empty lockfiles if lock file maintenance exists | 100 | not-applicable | — | — | Uses vi.spyOn and git.branchExists mock (git repo mock); not portable |

---


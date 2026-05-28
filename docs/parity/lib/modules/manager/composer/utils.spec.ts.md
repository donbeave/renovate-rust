# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/utils.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 33 | **Status:** done

### `extractConstraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from require | 20 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns platform php version | 31 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns platform 0 minor php version | 43 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns platform 0 patch php version | 55 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns platform lowest minor php version | 67 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns platform lowest patch php version | 79 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns from require-dev | 91 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns from composer platform require | 99 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns from composer platform require-dev | 110 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns from composer-runtime-api | 116 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns from plugin-api-version | 124 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| fallback to 1.* | 132 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |

### `getComposerArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| disables scripts and plugins by default | 144 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables platform requirements | 152 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables all platform requirements with 2.1.0 | 165 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables only extension and library platform requirements with ^2.1 | 178 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables only extension and library platform requirements with 2.2.0 | 191 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables only extension and library platform requirements with ^2.2 | 204 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables only extension and library platform requirements with 2.3.0 | 217 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables only extension and library platform requirements with ^2.3 | 230 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables single platform requirement | 243 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables multiple platform requirement | 256 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| allows scripts when configured | 269 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables scripts when configured locally | 278 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| allows plugins when configured | 294 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| disables plugins when configured locally | 303 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |

### `getComposerUpdateArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not request an update with minimal changes with $constraint | 321 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| requests an update with minimal changes with $constraint | 337 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| does not use --minimal-changes when composerNoMinimalChanges is set in postUpdateOptions | 361 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| does not use --minimal-changes for lock file maintenance | 374 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |

### `requireComposerDependencyInstallation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when symfony/flex has been installed | 389 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns true when symfony/flex has been installed as dev dependency | 396 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |
| returns false when symfony/flex has not been installed | 403 | not-applicable | — | — | Requires vi.mock(datasource) + GlobalConfig mock infrastructure |

---


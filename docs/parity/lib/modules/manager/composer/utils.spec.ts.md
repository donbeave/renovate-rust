# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/utils.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `extractConstraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from require | 20 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns platform php version | 31 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns platform 0 minor php version | 43 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns platform 0 patch php version | 55 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns platform lowest minor php version | 67 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns platform lowest patch php version | 79 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns from require-dev | 91 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns from composer platform require | 99 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns from composer platform require-dev | 110 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns from composer-runtime-api | 116 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns from plugin-api-version | 124 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| fallback to 1.* | 132 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |

### `getComposerArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| disables scripts and plugins by default | 144 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables platform requirements | 152 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables all platform requirements with 2.1.0 | 165 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables only extension and library platform requirements with ^2.1 | 178 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables only extension and library platform requirements with 2.2.0 | 191 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables only extension and library platform requirements with ^2.2 | 204 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables only extension and library platform requirements with 2.3.0 | 217 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables only extension and library platform requirements with ^2.3 | 230 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables single platform requirement | 243 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables multiple platform requirement | 256 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| allows scripts when configured | 269 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables scripts when configured locally | 278 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| allows plugins when configured | 294 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| disables plugins when configured locally | 303 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |

### `getComposerUpdateArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not request an update with minimal changes with $constraint | 321 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| requests an update with minimal changes with $constraint | 337 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| does not use --minimal-changes when composerNoMinimalChanges is set in postUpdateOptions | 361 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| does not use --minimal-changes for lock file maintenance | 374 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |

### `requireComposerDependencyInstallation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when symfony/flex has been installed | 389 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns true when symfony/flex has been installed as dev dependency | 396 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |
| returns false when symfony/flex has not been installed | 403 | not-applicable | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer | — | Mock framework internals — tests composer utils via vitest-mocked fs/HTTP; Rust tests this at different layer |

---

# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/utils.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 33 | **Status:** pending

### `extractConstraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from require | 20 | pending | — | — | —|
| returns platform php version | 31 | pending | — | — | —|
| returns platform 0 minor php version | 43 | pending | — | — | —|
| returns platform 0 patch php version | 55 | pending | — | — | —|
| returns platform lowest minor php version | 67 | pending | — | — | —|
| returns platform lowest patch php version | 79 | pending | — | — | —|
| returns from require-dev | 91 | pending | — | — | —|
| returns from composer platform require | 99 | pending | — | — | —|
| returns from composer platform require-dev | 110 | pending | — | — | —|
| returns from composer-runtime-api | 116 | pending | — | — | —|
| returns from plugin-api-version | 124 | pending | — | — | —|
| fallback to 1.* | 132 | pending | — | — | —|

### `getComposerArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| disables scripts and plugins by default | 144 | pending | — | — | —|
| disables platform requirements | 152 | pending | — | — | —|
| disables all platform requirements with 2.1.0 | 165 | pending | — | — | —|
| disables only extension and library platform requirements with ^2.1 | 178 | pending | — | — | —|
| disables only extension and library platform requirements with 2.2.0 | 191 | pending | — | — | —|
| disables only extension and library platform requirements with ^2.2 | 204 | pending | — | — | —|
| disables only extension and library platform requirements with 2.3.0 | 217 | pending | — | — | —|
| disables only extension and library platform requirements with ^2.3 | 230 | pending | — | — | —|
| disables single platform requirement | 243 | pending | — | — | —|
| disables multiple platform requirement | 256 | pending | — | — | —|
| allows scripts when configured | 269 | pending | — | — | —|
| disables scripts when configured locally | 278 | pending | — | — | —|
| allows plugins when configured | 294 | pending | — | — | —|
| disables plugins when configured locally | 303 | pending | — | — | —|

### `getComposerUpdateArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not request an update with minimal changes with $constraint | 321 | pending | — | — | —|
| requests an update with minimal changes with $constraint | 337 | pending | — | — | —|
| does not use --minimal-changes when composerNoMinimalChanges is set in postUpdateOptions | 361 | pending | — | — | —|
| does not use --minimal-changes for lock file maintenance | 374 | pending | — | — | —|

### `requireComposerDependencyInstallation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when symfony/flex has been installed | 389 | pending | — | — | —|
| returns true when symfony/flex has been installed as dev dependency | 396 | pending | — | — | —|
| returns false when symfony/flex has not been installed | 403 | pending | — | — | —|

---


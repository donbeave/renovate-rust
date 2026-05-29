# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/utils.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 33 | **Status:** not-applicable

### `extractConstraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from require | 20 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns platform php version | 31 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns platform 0 minor php version | 43 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns platform 0 patch php version | 55 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns platform lowest minor php version | 67 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns platform lowest patch php version | 79 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns from require-dev | 91 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns from composer platform require | 99 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns from composer platform require-dev | 110 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns from composer-runtime-api | 116 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns from plugin-api-version | 124 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| fallback to 1.* | 132 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|

### `getComposerArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| disables scripts and plugins by default | 144 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables platform requirements | 152 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables all platform requirements with 2.1.0 | 165 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables only extension and library platform requirements with ^2.1 | 178 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables only extension and library platform requirements with 2.2.0 | 191 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables only extension and library platform requirements with ^2.2 | 204 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables only extension and library platform requirements with 2.3.0 | 217 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables only extension and library platform requirements with ^2.3 | 230 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables single platform requirement | 243 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables multiple platform requirement | 256 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| allows scripts when configured | 269 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables scripts when configured locally | 278 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| allows plugins when configured | 294 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| disables plugins when configured locally | 303 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|

### `getComposerUpdateArguments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not request an update with minimal changes with $constraint | 321 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| requests an update with minimal changes with $constraint | 337 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| does not use --minimal-changes when composerNoMinimalChanges is set in postUpdateOptions | 361 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| does not use --minimal-changes for lock file maintenance | 374 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|

### `requireComposerDependencyInstallation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when symfony/flex has been installed | 389 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns true when symfony/flex has been installed as dev dependency | 396 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|
| returns false when symfony/flex has not been installed | 403 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript Composer utility pipeline|

---


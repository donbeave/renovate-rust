# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/azure-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/azure-helper.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** not-applicable

### `getRef`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get the ref with short ref name | 23 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should not get ref | 34 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should get the ref with full ref name | 45 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|

### `getAzureBranchObj`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get the branch object | 58 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should get the branch object when ref missing | 73 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|

### `getFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null error GitItemNotFoundException | 86 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return null error GitUnresolvableToCommitException | 115 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return the file content because it is not a json | 144 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return null because the file is not readable | 173 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|

### `getCommitDetails`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get commit details | 193 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|

### `getMergeMethod`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should default to NoFastForward | 208 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return NoFastForward when policy explicitly set | 220 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return RebaseMerge | 246 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return Squash | 272 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return Squash when Project wide exact branch policy exists | 298 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return default branch policy | 327 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return most specific exact branch policy | 366 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|
| should return most specific prefix branch policy | 435 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|

### `getAllProjectTeams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get all teams | 493 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK helper; TypeScript Azure DevOps helper pipeline|

---


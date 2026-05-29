# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/forgejo/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** pending

### `workers/repository/update/pr/changelog/forgejo/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 56 | pending | — | — | —|
| returns null if currentVersion equals newVersion | 65 | pending | — | — | —|
| skips invalid repos | 75 | pending | — | — | —|
| works without forgejo | 84 | pending | — | — | —|
| uses forgejo tags | 111 | pending | — | — | —|
| handles empty forgejo tags response | 224 | pending | — | — | —|
| uses forgejo tags with error | 259 | pending | — | — | —|
| handles no sourceUrl | 294 | pending | — | — | —|
| handles invalid sourceUrl | 303 | pending | — | — | —|
| handles no releases | 312 | pending | — | — | —|
| handles not enough releases | 321 | pending | — | — | —|
| supports self-hosted forgejo changelog | 330 | pending | — | — | —|

### `workers/repository/update/pr/changelog/forgejo/index › hasValidRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles invalid repository | 367 | pending | — | — | —|
| handles valid repository | 372 | pending | — | — | —|

### `workers/repository/update/pr/changelog/forgejo/index › getAllTags`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles endpoint | 378 | pending | — | — | —|

### `workers/repository/update/pr/changelog/forgejo/index › getReleaseNotesMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 394 | pending | — | — | —|

---


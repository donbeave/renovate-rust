# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/fetch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/fetch.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** pending

### `workers/repository/process/fetch › fetchUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty deps | 21 | pending | — | — | —|
| handles ignored, skipped and disabled | 31 | pending | — | — | —|
| fetches updates | 85 | pending | — | — | —|

### `workers/repository/process/fetch › fetchUpdates() › constraintsVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is merged from packageFile with config | 119 | pending | — | — | —|
| is set from packageFile if only set on packageFile | 147 | pending | — | — | —|
| is not set if neither config nor packageFile are set | 168 | pending | — | — | —|
| is set if config is set | 189 | pending | — | — | —|
| skips deps with empty names | 211 | pending | — | — | —|
| skips internal deps by default | 238 | pending | — | — | —|
| fetch updates for internal deps if updateInternalDeps is true | 261 | pending | — | — | —|
| throws lookup errors for onboarded repos | 283 | pending | — | — | —|
| throws lookup errors for not onboarded repos | 300 | pending | — | — | —|
| produces external host warnings for not onboarded repos | 317 | pending | — | — | —|

---


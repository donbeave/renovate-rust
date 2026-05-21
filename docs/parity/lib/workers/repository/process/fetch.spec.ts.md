# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/fetch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/fetch.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `workers/repository/process/fetch › fetchUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty deps | 21 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| handles ignored, skipped and disabled | 31 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| fetches updates | 85 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |

### `workers/repository/process/fetch › fetchUpdates() › constraintsVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is merged from packageFile with config | 119 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| is set from packageFile if only set on packageFile | 147 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| is not set if neither config nor packageFile are set | 168 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| is set if config is set | 189 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| skips deps with empty names | 211 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| skips internal deps by default | 238 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| fetch updates for internal deps if updateInternalDeps is true | 261 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| throws lookup errors for onboarded repos | 283 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| throws lookup errors for not onboarded repos | 300 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |
| produces external host warnings for not onboarded repos | 317 | not-applicable | — | — | tests dependency data fetching via datasource HTTP calls; datasource HTTP layer out of scope |

---


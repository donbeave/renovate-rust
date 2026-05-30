# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/updates/flatten.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/updates/flatten.spec.ts
**Total tests:** 6 | **Ported:** 1 | **Actionable:** 6 | **Status:** partial-applicable (remaining)

### `workers/repository/updates/flatten › sanitizeDepName()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitizes urls | 20 | ported | `branch.rs` | `sanitize_url_style_dep` | — |

### `workers/repository/updates/flatten › flattenUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| flattens | 28 | pending | — | — | — |
| when a dependency is enabled=false, it is filtered | 241 | pending | — | — | — |
| when a skipReason is found on a dependency, it is filtered | 280 | pending | — | — | — |
| separates lockfile maintenance updates from other update types if grouping is applied | 319 | pending | — | — | — |

### `workers/repository/updates/flatten › flattenUpdates() › hasAttestation is taken from the current value`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| current attestation %s, new attestation %s | 362 | pending | — | — | — |

---


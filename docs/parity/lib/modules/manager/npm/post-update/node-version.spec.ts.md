# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/node-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/node-version.spec.ts
**Total tests:** 11 | **Ported:** 2 | **Actionable:** 11 | **Status:** partial

### `getNodeConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from user constraints | 18 | pending | — | — | — |
| returns .node-version value | 29 | pending | — | — | — |
| returns .nvmrc value | 41 | pending | — | — | — |
| ignores unusable ranges in dotfiles | 52 | pending | — | — | — |
| returns from package.json | 64 | pending | — | — | — |
| returns from package.json volta | 74 | pending | — | — | — |
| prefers volta over engines | 84 | pending | — | — | — |

### `getNodeUpdate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns version | 101 | ported | `npm.rs` | `npm_get_node_update_returns_version` | — |
| returns undefined | 107 | ported | `npm.rs` | `npm_get_node_update_returns_none_for_empty` | — |

### `getNodeToolConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns getNodeUpdate | 113 | pending | — | — | — |
| returns getNodeConstraint | 127 | pending | — | — | — |

---


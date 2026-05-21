# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/node-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/node-version.spec.ts
**Total tests:** 11 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `getNodeConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from user constraints | 18 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |
| returns .node-version value | 29 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |
| returns .nvmrc value | 41 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |
| ignores unusable ranges in dotfiles | 52 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |
| returns from package.json | 64 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |
| returns from package.json volta | 74 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |
| prefers volta over engines | 84 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |

### `getNodeUpdate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns version | 101 | ported | `npm.rs` | `npm_get_node_update_returns_version` | — |
| returns undefined | 107 | ported | `npm.rs` | `npm_get_node_update_returns_none_for_empty` | — |

### `getNodeToolConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns getNodeUpdate | 113 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |
| returns getNodeConstraint | 127 | not-applicable | — | — | tests Node.js version detection via exec infrastructure; external tool invocation out of scope |

---


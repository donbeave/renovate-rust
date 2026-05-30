# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/homebrew/handlers/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homebrew/handlers/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `findHandlerByType`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for handler type "%s" (unknown, "") | 5 | ported | `homebrew.rs` | `find_handler_by_type_unknown_returns_none`, `find_handler_by_type_empty_returns_none` | — |
| returns github handler for github type | 9 | ported | `homebrew.rs` | `find_handler_by_type_github` | — |

### `findHandler`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for null URL | 16 | ported | `homebrew.rs` | `find_handler_none_url_returns_none` | — |
| returns null for unsupported URL | 20 | ported | `homebrew.rs` | `find_handler_unsupported_url_returns_none` | — |
| returns handler and parsed result for GitHub URL | 24 | ported | `homebrew.rs` | `find_handler_github_url` | — |

---


# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/body/notes.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/body/notes.spec.ts
**Total tests:** 3 | **Ported:** 1 | **Actionable:** 1 | **Status:** done

### `workers/repository/update/pr/body/notes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renders notes | 8 | not-applicable | — | — | `getPrNotes` uses Handlebars template engine (`template.compile`) which is not yet ported to Rust |
| handles render error | 25 | not-applicable | — | — | Same reason as line 8 |
| handles extra notes | 44 | ported | `branch.rs` | `get_pr_extra_notes_returns_relevant_strings` | — |

---


# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/body/notes.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/body/notes.spec.ts
**Total tests:** 3 | **Ported:** 1 | **Actionable:** 0 | **Status:** done

### `workers/repository/update/pr/body/notes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renders notes  | 8 | not-applicable | — | — | TS-library-specific schema internals — Handlebars template rendering mocked via vitest; no Rust equivalent |
| handles render error  | 25 | not-applicable | — | — | TS-library-specific schema internals — Handlebars template rendering mocked via vitest; no Rust equivalent |
| handles extra notes | 44 | ported | `branch.rs` | `get_pr_extra_notes_returns_relevant_strings` | — |

---


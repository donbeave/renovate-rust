# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/body/updates-table.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/body/updates-table.spec.ts
**Total tests:** 5 | **Ported:** 1 | **Actionable:** 0 | **Status:** done

### `workers/repository/update/pr/body/updates-table`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checks a case where prBodyColumns are undefined | 6 | ported | `branch.rs` | `get_pr_updates_table_returns_empty_without_columns` | — |
| checks results for getPrUpdatesTable  | 18 | not-applicable | — | — | TS-library-specific schema internals — tests Handlebars template rendering in table cells; no Rust equivalent |
| selects the best upgrade in case of duplicate table rows  | 155 | not-applicable | — | — | TS-library-specific schema internals — tests Handlebars template rendering in table cells; no Rust equivalent |
| handles replacements with new names  | 257 | not-applicable | — | — | TS-library-specific schema internals — tests Handlebars template rendering in table cells; no Rust equivalent |
| customizes table headers as per prBodyHeadingDefinitions  | 318 | not-applicable | — | — | TS-library-specific schema internals — tests Handlebars template rendering in table headers; no Rust equivalent |

---

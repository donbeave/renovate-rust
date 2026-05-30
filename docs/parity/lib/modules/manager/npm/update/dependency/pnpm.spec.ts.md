# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/dependency/pnpm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/dependency/pnpm.spec.ts
**Total tests:** 24 | **Ported:** 24 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null on invalid input | 8 | ported | `extractors/npm.rs` | `pnpm_update_dep_null_on_invalid` | — |
| handles implicit default catalog dependency | 19 | ported | `extractors/npm.rs` | `pnpm_update_dep_implicit_default_catalog` | — |
| handles explicit default catalog dependency | 46 | ported | `extractors/npm.rs` | `pnpm_update_dep_explicit_default_catalog` | — |
| handles explicit named catalog dependency | 75 | ported | `extractors/npm.rs` | `pnpm_update_dep_named_catalog` | — |
| does nothing if the new and old values match | 111 | ported | `extractors/npm.rs` | `pnpm_update_dep_already_at_version` | — |
| replaces package | 132 | ported | `extractors/npm.rs` | `pnpm_update_dep_replaces_package` | — |
| replaces a github dependency value | 160 | ported | `extractors/npm.rs` | `pnpm_update_dep_github_value` | — |
| replaces a npm package alias | 189 | ported | `extractors/npm.rs` | `pnpm_update_dep_npm_alias` | — |
| replaces a github short hash | 219 | ported | `extractors/npm.rs` | `pnpm_update_dep_short_hash` | — |
| replaces a github fully specified version | 248 | ported | `extractors/npm.rs` | `pnpm_update_dep_git_tag` | — |
| returns null if the dependency is not present in the target catalog | 277 | ported | `extractors/npm.rs` | `pnpm_update_dep_null_if_not_in_catalog` | — |
| returns null if catalogs are missing | 298 | ported | `extractors/npm.rs` | `pnpm_update_dep_null_if_no_catalog` | — |
| returns null if empty file | 316 | ported | `extractors/npm.rs` | `pnpm_update_dep_null_on_empty` | — |
| preserves literal whitespace | 330 | ported | `extractors/npm.rs` | `pnpm_update_dep_preserves_whitespace` | — |
| preserves single quote style | 357 | ported | `extractors/npm.rs` | `pnpm_update_dep_preserves_single_quotes` | — |
| preserves comments | 384 | ported | `extractors/npm.rs` | `pnpm_update_dep_preserves_comments` | — |
| preserves double quote style | 415 | ported | `extractors/npm.rs` | `pnpm_update_dep_preserves_double_quotes` | — |
| preserves anchors, replacing only the value | 442 | ported | `extractors/npm.rs` | `pnpm_update_dep_preserves_anchors` | — |
| preserves whitespace with anchors | 474 | ported | `extractors/npm.rs` | `pnpm_update_dep_preserves_anchor_whitespace` | — |
| preserves quotation style with anchors | 501 | ported | `extractors/npm.rs` | `pnpm_update_dep_preserves_anchor_quote_style` | — |
| preserves formatting in flow style syntax | 528 | ported | `extractors/npm.rs` | `pnpm_update_dep_flow_style` | — |
| does not replace aliases in the value position | 559 | ported | `extractors/npm.rs` | `pnpm_update_dep_no_replace_value_alias` | — |
| does not replace aliases in the key position | 587 | ported | `extractors/npm.rs` | `pnpm_update_dep_no_replace_key_alias` | — |
| handles workspace overrides | 611 | ported | `extractors/npm.rs` | `pnpm_update_dep_workspace_overrides` | — |

---

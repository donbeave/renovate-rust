# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/dependency/yarn.spec.ts
**Total tests:** 26 | **Ported:** 25 | **Actionable:** 0 | **Status:** done

### `updateYarnrcCatalogDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if catalogName is missing and logs error | 8 | ported | `extractors/npm.rs` | `yarn_update_dep_null_on_missing_catalog_name` | — |
| ensure continuation even if catalog list and update does not match | 33 | ported | `extractors/npm.rs` | `yarn_update_dep_null_catalog_mismatch` | — |
| ensure continuation even if dependency and update does not match | 55 | ported | `extractors/npm.rs` | `yarn_update_dep_null_dep_mismatch` | — |
| ensure trace logging | 78 | not-applicable | — | — | Tests TypeScript mock logger infrastructure; trace logging not testable via Rust tracing in unit tests |

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if catalogName is missing | 103 | ported | `extractors/npm.rs` | `yarn_update_dep_null_missing_dep_type` | — |
| handles implicit default catalog dependency | 125 | ported | `extractors/npm.rs` | `yarn_update_dep_implicit_default_catalog` | — |
| handles explicit named catalog dependency | 150 | ported | `extractors/npm.rs` | `yarn_update_dep_named_catalog` | — |
| does nothing if the new and old values match | 177 | ported | `extractors/npm.rs` | `yarn_update_dep_already_at_version` | — |
| replaces package | 197 | ported | `extractors/npm.rs` | `yarn_update_dep_replaces_package` | — |
| replaces a github dependency value | 224 | ported | `extractors/npm.rs` | `yarn_update_dep_github_value` | — |
| replaces a npm package alias | 251 | ported | `extractors/npm.rs` | `yarn_update_dep_npm_alias` | — |
| replaces a github short hash | 279 | ported | `extractors/npm.rs` | `yarn_update_dep_short_hash` | — |
| replaces a github fully specified version | 307 | ported | `extractors/npm.rs` | `yarn_update_dep_git_tag` | — |
| returns null if the dependency is not present in the target catalog | 332 | ported | `extractors/npm.rs` | `yarn_update_dep_null_not_in_catalog` | — |
| returns null if catalogs are missing | 352 | ported | `extractors/npm.rs` | `yarn_update_dep_null_no_catalog` | — |
| returns null if empty file | 372 | ported | `extractors/npm.rs` | `yarn_update_dep_null_on_empty` | — |
| preserves literal whitespace | 388 | ported | `extractors/npm.rs` | `yarn_update_dep_preserves_whitespace` | — |
| preserves single quote style | 414 | ported | `extractors/npm.rs` | `yarn_update_dep_preserves_single_quotes` | — |
| preserves comments | 437 | ported | `extractors/npm.rs` | `yarn_update_dep_preserves_comments` | — |
| preserves double quote style | 467 | ported | `extractors/npm.rs` | `yarn_update_dep_preserves_double_quotes` | — |
| preserves anchors, replacing only the value | 492 | ported | `extractors/npm.rs` | `yarn_update_dep_preserves_anchors` | — |
| preserves whitespace with anchors | 521 | ported | `extractors/npm.rs` | `yarn_update_dep_preserves_anchor_whitespace` | — |
| preserves quotation style with anchors | 547 | ported | `extractors/npm.rs` | `yarn_update_dep_preserves_anchor_quote_style` | — |
| preserves formatting in flow style syntax | 575 | ported | `extractors/npm.rs` | `yarn_update_dep_flow_style` | — |
| does not replace aliases in the value position | 605 | ported | `extractors/npm.rs` | `yarn_update_dep_no_replace_value_alias` | — |
| does not replace aliases in the key position | 631 | ported | `extractors/npm.rs` | `yarn_update_dep_no_replace_key_alias` | — |

---

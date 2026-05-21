# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/update.spec.ts
**Total tests:** 38 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateDependency › deno.json/jsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates dependency in imports | 9 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| throws when multiple imports require more than one replacement | 39 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| updates dependency in scopes | 64 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when scopes element not found | 98 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| updates dependency in tasks | 127 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| updates dependency in tasks.command | 158 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when tasks element not found | 191 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when tasks.command element not found | 221 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| updates dependency in compilerOptions.types | 251 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when compilerOptions.types is empty array | 281 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when compilerOptions.types element not found | 308 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| updates dependency in compilerOptions.jsxImportSource | 335 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when compilerOptions.jsxImportSource does not exist | 367 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when compilerOptions.jsxImportSourceTypes does not exist | 394 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| updates dependency in compilerOptions.jsxImportSourceTypes | 421 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| updates dependency in lint plugins | 453 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when lint.plugins element not found | 481 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when lint.plugins is empty array | 508 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| handles dependency without version | 535 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null if packageFile is not defined | 563 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null for not supported datasource | 575 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| currentValue is not defined when deno datasource | 602 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null for missing required values | 629 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| handles complex JSON with nested structures | 648 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| handles the case where the desired version is already supported | 689 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null if empty file | 712 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| handles error during update gracefully | 731 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| depName is not defined | 750 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| unsupported packageFile | 773 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| replaces only exact matches | 792 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when depType is not handled | 818 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when compilerOptions.types does not exist | 841 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null when lint.plugins does not exist | 864 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |

### `updateDependency › <importMap>.json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates dependency in imports | 889 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| handles error during update gracefully | 916 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| returns null for not supported datasource | 938 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |
| depName is not defined | 968 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |

### `updateDependency › package.json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces a dependency value | 997 | not-applicable | — | — | tests Deno import map URL updates; needs full Deno import resolution infrastructure |

---


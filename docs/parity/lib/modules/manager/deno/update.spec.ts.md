# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/update.spec.ts
**Total tests:** 38 | **Ported:** 0 | **Actionable:** 38 | **Status:** pending

### `updateDependency › deno.json/jsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates dependency in imports | 9 | pending | — | — | — |
| throws when multiple imports require more than one replacement | 39 | pending | — | — | — |
| updates dependency in scopes | 64 | pending | — | — | — |
| returns null when scopes element not found | 98 | pending | — | — | — |
| updates dependency in tasks | 127 | pending | — | — | — |
| updates dependency in tasks.command | 158 | pending | — | — | — |
| returns null when tasks element not found | 191 | pending | — | — | — |
| returns null when tasks.command element not found | 221 | pending | — | — | — |
| updates dependency in compilerOptions.types | 251 | pending | — | — | — |
| returns null when compilerOptions.types is empty array | 281 | pending | — | — | — |
| returns null when compilerOptions.types element not found | 308 | pending | — | — | — |
| updates dependency in compilerOptions.jsxImportSource | 335 | pending | — | — | — |
| returns null when compilerOptions.jsxImportSource does not exist | 367 | pending | — | — | — |
| returns null when compilerOptions.jsxImportSourceTypes does not exist | 394 | pending | — | — | — |
| updates dependency in compilerOptions.jsxImportSourceTypes | 421 | pending | — | — | — |
| updates dependency in lint plugins | 453 | pending | — | — | — |
| returns null when lint.plugins element not found | 481 | pending | — | — | — |
| returns null when lint.plugins is empty array | 508 | pending | — | — | — |
| handles dependency without version | 535 | pending | — | — | — |
| returns null if packageFile is not defined | 563 | pending | — | — | — |
| returns null for not supported datasource | 575 | pending | — | — | — |
| currentValue is not defined when deno datasource | 602 | pending | — | — | — |
| returns null for missing required values | 629 | pending | — | — | — |
| handles complex JSON with nested structures | 648 | pending | — | — | — |
| handles the case where the desired version is already supported | 689 | pending | — | — | — |
| returns null if empty file | 712 | pending | — | — | — |
| handles error during update gracefully | 731 | pending | — | — | — |
| depName is not defined | 750 | pending | — | — | — |
| unsupported packageFile | 773 | pending | — | — | — |
| replaces only exact matches | 792 | pending | — | — | — |
| returns null when depType is not handled | 818 | pending | — | — | — |
| returns null when compilerOptions.types does not exist | 841 | pending | — | — | — |
| returns null when lint.plugins does not exist | 864 | pending | — | — | — |

### `updateDependency › <importMap>.json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates dependency in imports | 889 | pending | — | — | — |
| handles error during update gracefully | 916 | pending | — | — | — |
| returns null for not supported datasource | 938 | pending | — | — | — |
| depName is not defined | 968 | pending | — | — | — |

### `updateDependency › package.json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces a dependency value | 997 | pending | — | — | — |

---


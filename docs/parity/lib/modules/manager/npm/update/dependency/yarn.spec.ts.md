# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/dependency/yarn.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** pending

### `updateYarnrcCatalogDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if catalogName is missing and logs error | 8 | not-applicable | — | — | Asserts expect(logger.logger.error).toHaveBeenCalledWith — logger spy |
| ensure continuation even if catalog list and update does not match | 33 | pending | — | — | — |
| ensure continuation even if dependency and update does not match | 55 | pending | — | — | — |
| ensure trace logging | 78 | not-applicable | — | — | Asserts expect(logger.logger.trace).toHaveBeenCalledWith — logger spy |

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if catalogName is missing | 103 | pending | — | — | — |
| handles implicit default catalog dependency | 125 | pending | — | — | — |
| handles explicit named catalog dependency | 150 | pending | — | — | — |
| does nothing if the new and old values match | 177 | pending | — | — | — |
| replaces package | 197 | pending | — | — | — |
| replaces a github dependency value | 224 | pending | — | — | — |
| replaces a npm package alias | 251 | pending | — | — | — |
| replaces a github short hash | 279 | pending | — | — | — |
| replaces a github fully specified version | 306 | pending | — | — | — |
| returns null if the dependency is not present in the target catalog | 334 | pending | — | — | — |
| returns null if catalogs are missing | 357 | pending | — | — | — |
| returns null if empty file | 375 | pending | — | — | — |
| preserves literal whitespace | 389 | pending | — | — | — |
| preserves single quote style | 415 | pending | — | — | — |
| preserves comments | 440 | pending | — | — | — |
| preserves double quote style | 469 | pending | — | — | — |
| preserves anchors, replacing only the value | 494 | pending | — | — | — |
| preserves whitespace with anchors | 524 | pending | — | — | — |
| preserves quotation style with anchors | 549 | pending | — | — | — |
| preserves formatting in flow style syntax | 574 | pending | — | — | — |
| does not replace aliases in the value position | 603 | pending | — | — | — |
| does not replace aliases in the key position | 630 | pending | — | — | — |

---


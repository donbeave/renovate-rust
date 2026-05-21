# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/dependency/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/dependency/yarn.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateYarnrcCatalogDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if catalogName is missing and logs error | 8 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| ensure continuation even if catalog list and update does not match | 33 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| ensure continuation even if dependency and update does not match | 55 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| ensure trace logging | 78 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if catalogName is missing | 103 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| handles implicit default catalog dependency | 125 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| handles explicit named catalog dependency | 150 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| does nothing if the new and old values match | 177 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| replaces package | 197 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| replaces a github dependency value | 224 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| replaces a npm package alias | 251 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| replaces a github short hash | 279 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| replaces a github fully specified version | 306 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| returns null if the dependency is not present in the target catalog | 334 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| returns null if catalogs are missing | 357 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| returns null if empty file | 375 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves literal whitespace | 389 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves single quote style | 415 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves comments | 440 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves double quote style | 469 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves anchors, replacing only the value | 494 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves whitespace with anchors | 524 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves quotation style with anchors | 549 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| preserves formatting in flow style syntax | 574 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| does not replace aliases in the value position | 603 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |
| does not replace aliases in the key position | 630 | not-applicable | — | — | tests yarn.lock dependency update via lockfile manipulation; complex lockfile format out of scope |

---

